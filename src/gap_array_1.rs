use std::mem;

use crate::Map;

enum Node<K, V> {
    Pair { key: K, value: V },
    Gap { next: usize }, // next > 0...
}

pub struct GapArray1<K, V> {
    nodes: Box<[Node<K, V>]>,
    len: usize,
}

impl<K: Ord, V> GapArray1<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Box::new([
                Node::Gap { next: 4 },
                Node::Gap { next: 2 },
                Node::Gap { next: 4 },
                Node::Gap { next: 4 },
            ]),
            len: 0,
        }
    }

    fn find(&self, key: &K) -> (bool, usize) {
        let mut from = 0;
        let mut to = self.nodes.len() - 1;
        while from < to {
            let mut middle = (from + to) / 2;
            while let Node::Gap { next } = self.nodes[middle] {
                if next > to {
                    to = (from + to) / 2;
                    if to <= from {
                        return (false, from);
                    }
                    middle = (from + to) / 2;
                } else {
                    middle = next;
                }
            }
            if let Node::Pair { key: k, value: _ } = &self.nodes[middle] {
                match k.cmp(key) {
                    std::cmp::Ordering::Less => from = middle + 1,
                    std::cmp::Ordering::Equal => return (true, middle),
                    std::cmp::Ordering::Greater => to = (from + to) / 2,
                }
            }
        }
        (false, from)
    }

    fn update_next(&mut self, index: usize) {
        let mut i = index;
        while i > 0 {
            i -= 1 << i.trailing_zeros();
            if let Node::Gap { next } = &self.nodes[i] {
                if next > &index {
                    self.nodes[i] = Node::Gap { next: index };
                    continue;
                }
            }
            return;
        }
    }

    fn resize(&mut self, new_nodes_len: usize) {
        // does this make sense?
        let nodes: Vec<Node<K, V>> = (0..new_nodes_len)
            .map(|i| Node::Gap {
                next: if i == 0 {
                    new_nodes_len
                } else {
                    i + 1 << i.trailing_zeros()
                },
            })
            .collect();
        let mut nodes = mem::replace(&mut self.nodes, nodes.into_boxed_slice());

        // rounding up is vital
        let m = 1 + (new_nodes_len - 1) / (new_nodes_len - self.len);
        // as is keeping the last spot open atm
        let indices: Vec<usize> = (0..new_nodes_len)
            .filter(|i| (new_nodes_len - i) % m != 1)
            .collect();
        // println!(
        //     "distro {} {} {} {:?}",
        //     new_nodes_len,
        //     self.len,
        //     indices.len(),
        //     indices
        // );
        // aim for an even distribution
        // so each element so go at new_nodes_len/len
        let mut i = 0;
        for j in 0..nodes.len() {
            if let Node::Gap { next: _ } = nodes[j] {
                continue;
            }
            let index = indices[i];
            self.nodes[index] = mem::replace(&mut nodes[j], Node::Gap { next: 0 });
            self.update_next(index);
            i += 1;
        }
    }

    fn push(&mut self, mut node: Node<K, V>, index: usize) -> Option<Node<K, V>> {
        let to = (index + self.nodes.len().ilog2() as usize).min(self.nodes.len());
        for i in index..to as usize {
            match self.nodes[i] {
                Node::Pair { key: _, value: _ } => node = mem::replace(&mut self.nodes[i], node),
                Node::Gap { next: _ } => {
                    self.nodes[i] = node;
                    self.update_next(i);
                    return None;
                }
            }
        }
        Some(node)
    }

    fn required_capacity_for_add(&self) -> usize {
        if self.len < 2 {
            return 4;
        }
        let new_len = self.len + 1;
        (new_len + new_len / new_len.ilog2() as usize).next_power_of_two()
    }
}

impl<K: Ord, V> Map<K, V> for GapArray1<K, V> {
    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn contains_key(&self, key: &K) -> bool {
        self.find(key).0
    }

    fn get(&self, key: &K) -> Option<&V> {
        let (matched, index) = self.find(key);
        if matched && let Node::Pair { key: _, value: v } = &self.nodes[index] {
            Some(v)
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let required_capacity = self.required_capacity_for_add();
        if self.nodes.len() < required_capacity {
            self.resize(required_capacity)
        }

        let (matched, index) = self.find(&key);

        // first simple case
        if matched {
            // hmm...
            if let Node::Pair { key: _, value } =
                mem::replace(&mut self.nodes[index], Node::Pair { key, value })
            {
                return Some(value);
            }
            // should never happen
            return None;
        }
        self.len += 1;
        if let Some(Node::Pair { key, value }) = self.push(Node::Pair { key, value }, index) {
            // show assure that there is room for the expelled node nearby
            self.resize(self.nodes.len());
            let (_, index) = self.find(&key);
            // but it turn out there f'in isn't. why not?
            assert!(self.push(Node::Pair { key, value }, index).is_none());
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let (matched, index) = self.find(key);
        if !matched {
            return None;
        }
        // determine search range
        let to = if index == 0 {
            self.nodes.len()
        } else {
            index + (1 << index.trailing_zeros())
        };
        let mut bottom = index + 1;
        while bottom < to {
            match &self.nodes[bottom] {
                Node::Gap { next: b } => {
                    if b < &self.nodes.len() {
                        bottom = *b
                    } else {
                        bottom += 1
                    }
                }
                Node::Pair { key: _, value: _ } => break,
            }
        }
        if let Node::Pair { key: _, value } =
            mem::replace(&mut self.nodes[index], Node::Gap { next: bottom })
        {
            self.len -= 1;
            Some(value)
        } else {
            // should not happen
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GapArray1, Map};

    #[test]
    fn tree_operations_work() {
        let mut tree = GapArray1::new();
        assert!(tree.is_empty());

        assert_eq!(tree.insert(3, "three"), None);
        assert_eq!(tree.insert(1, "one"), None);
        assert_eq!(tree.insert(4, "four"), None);
        assert_eq!(tree.insert(3, "tres"), Some("three"));

        assert_eq!(tree.len(), 3);
        assert!(tree.contains_key(&1));
        assert_eq!(tree.get(&3), Some(&"tres"));
        assert_eq!(tree.get(&2), None);

        assert_eq!(tree.remove(&1), Some("one"));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.get(&3), Some(&"tres"));
        assert_eq!(tree.remove(&3), Some("tres"));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn add_many() {
        let keys: Vec<i32> = (0..500 as i32).collect::<Vec<i32>>().repeat(2);
        let mut map = GapArray1::<i32, i32>::new();
        for key in keys {
            map.insert(key, key);
        }
    }
}
