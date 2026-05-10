use std::{
    cmp::Ordering,
    mem::{self, MaybeUninit},
};

use crate::Map;

// instead of optional boxes, stick all nodes into a vec and use indices.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    color: Color,
    key: K,
    value: V,
    left: usize,
    right: usize,
    parent: usize,
}

pub struct ArrayRedBlackTree<K, V> {
    root: usize,
    nodes: Vec<Node<K, V>>,
}

impl<K, V> ArrayRedBlackTree<K, V> {
    const NIL: usize = usize::MAX;

    fn left(&self, node: usize) -> usize {
        if node == Self::NIL {
            Self::NIL
        } else {
            self.nodes[node].left
        }
    }

    fn right(&self, node: usize) -> usize {
        if node == Self::NIL {
            Self::NIL
        } else {
            self.nodes[node].right
        }
    }

    fn parent(&self, node: usize) -> usize {
        if node == Self::NIL {
            Self::NIL
        } else {
            self.nodes[node].parent
        }
    }

    fn is_red(&self, node: usize) -> bool {
        node != Self::NIL && self.nodes[node].color == Color::Red
    }

    fn rotate_left(&mut self, x: usize) {
        let y = self.right(x);
        if y == Self::NIL {
            return;
        }
        let z = self.left(y);
        self.nodes[x].right = z;
        self.nodes[y].left = x;
        if z != Self::NIL {
            self.nodes[z].parent = x;
        }
        let parent = self.parent(x);
        self.nodes[x].parent = y;
        self.nodes[y].parent = parent;
        if parent == Self::NIL {
            self.root = y;
        } else if x == self.right(parent) {
            self.nodes[parent].right = y;
        } else {
            self.nodes[parent].left = y;
        }
    }

    fn rotate_right(&mut self, x: usize) {
        let y = self.left(x);
        if y == Self::NIL {
            return;
        }
        let z = self.right(y);
        self.nodes[y].right = x;
        self.nodes[x].left = z;
        if z != Self::NIL {
            self.nodes[z].parent = x;
        }
        let parent = self.parent(x);
        self.nodes[x].parent = y;
        self.nodes[y].parent = parent;
        if parent == Self::NIL {
            self.root = y;
        } else if x == self.right(parent) {
            self.nodes[parent].right = y;
        } else {
            self.nodes[parent].left = y;
        }
    }

    fn fix_insert(&mut self, mut k: usize) {
        while k != self.root && self.is_red(self.parent(k)) {
            let mut parent = self.parent(k);
            if self.parent(k) == self.left(self.parent(parent)) {
                let uncle = self.right(self.parent(parent));
                if self.is_red(uncle) {
                    self.nodes[parent].color = Color::Black;
                    self.nodes[uncle].color = Color::Black;
                    k = self.parent(parent);
                    self.nodes[k].color = Color::Red;
                    continue;
                }
                if k == self.right(parent) {
                    k = parent;
                    self.rotate_left(k);
                    parent = self.parent(k);
                }
                self.nodes[parent].color = Color::Black;
                let grand_parent = self.parent(parent);
                self.nodes[grand_parent].color = Color::Red;
                self.rotate_right(grand_parent);
                continue;
            }
            let uncle = self.left(self.parent(parent));
            if self.is_red(uncle) {
                self.nodes[parent].color = Color::Black;
                self.nodes[uncle].color = Color::Black;
                k = self.parent(parent);
                self.nodes[k].color = Color::Red;
                continue;
            }
            if k == self.left(parent) {
                k = parent;
                self.rotate_right(k);
                parent = self.parent(k);
            }
            self.nodes[parent].color = Color::Black;
            let grand_parent = self.parent(parent);
            self.nodes[grand_parent].color = Color::Red;
            self.rotate_left(grand_parent);
        }
        self.nodes[self.root].color = Color::Black;
    }

    fn fix_remove(&mut self, mut x: usize, mut px: usize) {
        while px != Self::NIL && !self.is_red(x) {
            if x == self.left(px) {
                let mut w = self.right(px);
                if w == Self::NIL {
                    x = px;
                    px = self.parent(px);
                    continue;
                }
                if self.is_red(w) {
                    self.nodes[w].color = Color::Black;
                    self.nodes[px].color = Color::Red;
                    self.rotate_left(px);
                    w = self.right(px);
                }
                if (self.left(w) == Self::NIL || !self.is_red(self.left(w)))
                    && (self.right(w) == Self::NIL || !self.is_red(self.right(w)))
                {
                    self.nodes[w].color = Color::Red;
                    x = px;
                    px = self.parent(px);
                } else {
                    if self.right(w) == Self::NIL || !self.is_red(self.right(w)) {
                        let lw = self.left(w);
                        if lw != Self::NIL {
                            self.nodes[lw].color = Color::Black;
                        }
                        self.nodes[w].color = Color::Red;
                        self.rotate_right(w);
                        w = self.right(px);
                    }
                    self.nodes[w].color = self.nodes[px].color;
                    self.nodes[px].color = Color::Black;
                    let rw = self.right(w);
                    if rw != Self::NIL {
                        self.nodes[rw].color = Color::Black;
                    }
                    self.rotate_left(px);
                    x = self.root;
                    break;
                }
            } else {
                let mut w = self.left(px);
                if w == Self::NIL {
                    x = px;
                    px = self.parent(px);
                    continue;
                }
                if self.is_red(w) {
                    self.nodes[w].color = Color::Black;
                    self.nodes[px].color = Color::Red;
                    self.rotate_right(px);
                    w = self.left(px);
                }
                if (self.right(w) == Self::NIL || !self.is_red(self.right(w)))
                    && (self.left(w) == Self::NIL || !self.is_red(self.left(w)))
                {
                    self.nodes[w].color = Color::Red;
                    x = px;
                    px = self.parent(px);
                } else {
                    if self.left(w) == Self::NIL || !self.is_red(self.left(w)) {
                        let rw = self.right(w);
                        if rw != Self::NIL {
                            self.nodes[rw].color = Color::Black;
                        }
                        self.nodes[w].color = Color::Red;
                        self.rotate_left(w);
                        w = self.left(px);
                    }
                    self.nodes[w].color = self.nodes[px].color;
                    self.nodes[px].color = Color::Black;
                    let lw = self.left(w);
                    if lw != Self::NIL {
                        self.nodes[lw].color = Color::Black;
                    }
                    self.rotate_right(px);
                    x = self.root;
                    break;
                }
            }
        }
        if x != Self::NIL {
            self.nodes[x].color = Color::Black;
        }
    }

    fn new_node(&mut self, key: K, value: V, parent: usize) -> usize {
        self.nodes.push(Node {
            color: Color::Red,
            key,
            value,
            left: Self::NIL,
            right: Self::NIL,
            parent,
        });
        self.nodes.len() - 1
    }

    fn remove_node(&mut self, x: usize) -> (K, V) {
        if x == self.nodes.len() - 1 {
            let node = self.nodes.pop().unwrap();
            return (node.key, node.value);
        }
        let old_len = self.nodes.len();
        let swapped_parent = self.nodes[old_len - 1].parent;
        let swapped_left = self.nodes[old_len - 1].left;
        let swapped_right = self.nodes[old_len - 1].right;
        let node = self.nodes.swap_remove(x);
        // update swapped node's pointers to NIL since disconnected
        self.nodes[x].parent = Self::NIL;
        self.nodes[x].left = Self::NIL;
        self.nodes[x].right = Self::NIL;
        // update swapped's children parent
        if swapped_left != Self::NIL {
            self.nodes[swapped_left].parent = x;
        }
        if swapped_right != Self::NIL {
            self.nodes[swapped_right].parent = x;
        }
        // update swapped's parent child
        if swapped_parent != Self::NIL {
            if self.nodes[swapped_parent].left == old_len - 1 {
                self.nodes[swapped_parent].left = x;
            } else if self.nodes[swapped_parent].right == old_len - 1 {
                self.nodes[swapped_parent].right = x;
            }
        } else if self.root == old_len - 1 {
            self.root = x;
        }
        (node.key, node.value)
    }

    fn disconnect_least(&mut self, start: usize) -> (usize, K, V) {
        let mut least = start;
        while self.left(least) != Self::NIL {
            least = self.left(least);
        }
        let key = mem::replace(&mut self.nodes[least].key, unsafe {
            MaybeUninit::uninit().assume_init()
        });
        let value = self.disconnect(least);
        (least, key, value)
    }

    fn disconnect(&mut self, node: usize) -> V {
        let parent = self.parent(node);
        let child = if self.left(node) != Self::NIL {
            self.left(node)
        } else {
            self.right(node)
        };
        if child != Self::NIL {
            self.nodes[child].parent = parent;
        }
        if parent == Self::NIL {
            self.root = child;
        } else if self.left(parent) == node {
            self.nodes[parent].left = child;
        } else {
            self.nodes[parent].right = child;
        }
        if self.nodes[node].color == Color::Black {
            self.fix_remove(child, parent);
        }
        mem::replace(&mut self.nodes[node].value, unsafe {
            MaybeUninit::uninit().assume_init()
        })
    }

    pub fn new() -> Self {
        Self {
            root: Self::NIL,
            nodes: Vec::new(),
        }
    }
}

impl<K: Ord, V> Map<K, V> for ArrayRedBlackTree<K, V> {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut x = self.root;
        while x != Self::NIL {
            match self.nodes[x].key.cmp(key) {
                Ordering::Equal => return Some(&self.nodes[x].value),
                Ordering::Greater => x = self.right(x),
                Ordering::Less => x = self.left(x),
            }
        }
        None
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut parent = Self::NIL;
        let mut current = self.root;
        while current != Self::NIL {
            match self.nodes[current].key.cmp(&key) {
                Ordering::Equal => {
                    return Some(mem::replace(&mut self.nodes[current].value, value));
                }
                Ordering::Greater => {
                    parent = current;
                    current = self.right(current);
                }
                Ordering::Less => {
                    parent = current;
                    current = self.left(current);
                }
            }
        }
        let node = self.new_node(key, value, parent);
        if parent == Self::NIL {
            self.nodes[node].color = Color::Black;
            self.root = node;
            return None;
        }
        // comparison repeated...
        match self.nodes[parent].key.cmp(&self.nodes[node].key) {
            Ordering::Greater => self.nodes[parent].right = node,
            Ordering::Less => self.nodes[parent].left = node,
            Ordering::Equal => panic!("how did you get here!?"),
        }
        if self.parent(parent) == Self::NIL {
            return None;
        }
        self.fix_insert(node);
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut current = self.root;
        while current != Self::NIL {
            match self.nodes[current].key.cmp(key) {
                Ordering::Equal => break,
                Ordering::Greater => current = self.right(current),
                Ordering::Less => current = self.left(current),
            }
        }
        if current == Self::NIL {
            return None;
        }
        if self.left(current) != Self::NIL && self.right(current) != Self::NIL {
            let (succ_idx, succ_key, succ_value) = self.disconnect_least(self.right(current));
            self.nodes[current].key = succ_key;
            let value = succ_value;
            self.remove_node(succ_idx);
            Some(value)
        } else {
            let value = self.disconnect(current);
            self.remove_node(current);
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArrayRedBlackTree, Map};

    #[test]
    fn tree_operations_work() {
        let mut tree = ArrayRedBlackTree::new();
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
}
