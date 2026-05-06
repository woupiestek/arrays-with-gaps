use crate::Map;
use std::{cmp::Ordering, mem::MaybeUninit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

struct Nodes<K, V> {
    colors: Vec<Color>,
    keys: Vec<MaybeUninit<K>>,
    values: Vec<MaybeUninit<V>>,
    lefts: Vec<u32>,
    rights: Vec<u32>,
    free: Vec<u32>,
}

const SENTINEL: u32 = u32::MAX;

impl<K, V> Nodes<K, V> {
    fn new() -> Self {
        Self {
            colors: Vec::new(),
            keys: Vec::new(),
            values: Vec::new(),
            lefts: Vec::new(),
            rights: Vec::new(),
            free: Vec::new(),
        }
    }

    fn flip(&mut self, node: u32) {
        self.colors[node as usize] = match self.colors[node as usize] {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        }
    }

    fn add(&mut self, key: K, value: V) -> u32 {
        if let Some(node) = self.free.pop() {
            self.keys[node as usize].write(key);
            self.values[node as usize].write(value);
            self.lefts[node as usize] = SENTINEL;
            self.colors[node as usize] = Color::Red;
            self.rights[node as usize] = SENTINEL;
            node
        } else {
            self.colors.push(Color::Red);
            self.keys.push(MaybeUninit::new(key));
            self.values.push(MaybeUninit::new(value));
            self.lefts.push(SENTINEL);
            self.rights.push(SENTINEL);
            self.colors.len() as u32 - 1
        }
    }

    fn remove(&mut self, node: u32) -> (K, V) {
        self.free.push(node);
        unsafe {
            (
                self.keys[node as usize].assume_init_read(),
                self.values[node as usize].assume_init_read(),
            )
        }
    }

    fn is_red(&self, node: u32) -> bool {
        node < SENTINEL && self.colors[node as usize] == Color::Red
    }

    fn key(&self, node: u32) -> &K {
        unsafe { self.keys[node as usize].assume_init_ref() }
    }

    fn rekey(&mut self, node: u32, key: K) -> K {
        unsafe {
            let k = self.keys[node as usize].assume_init_read();
            self.keys[node as usize].write(key);
            k
        }
    }

    fn value(&self, node: u32) -> &V {
        unsafe { self.values[node as usize].assume_init_ref() }
    }

    fn revalue(&mut self, node: u32, value: V) -> V {
        unsafe {
            let v = self.values[node as usize].assume_init_read();
            self.values[node as usize].write(value);
            v
        }
    }

    fn left(&self, node: u32) -> u32 {
        if node == SENTINEL {
            SENTINEL
        } else {
            self.lefts[node as usize]
        }
    }

    fn right(&self, node: u32) -> u32 {
        if node == SENTINEL {
            SENTINEL
        } else {
            self.rights[node as usize]
        }
    }
}

pub struct SOARBTree<K, V> {
    nodes: Nodes<K, V>,
    root: u32,
    len: u32,
}

impl<K: Ord, V> Map<K, V> for SOARBTree<K, V> {
    fn len(&self) -> usize {
        self.len as usize
    }

    fn is_empty(&self) -> bool {
        self.root == SENTINEL
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get_node(self.root, key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut added = false;
        let (root, old_value) = self.insert_node(self.root, key, value, &mut added);
        self.root = root;
        self.nodes.colors[root as usize] = Color::Red;
        if added {
            self.len += 1;
        }
        old_value
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if self.root == SENTINEL {
            return None;
        }

        if !{ self.nodes.is_red(self.root) } && !{ self.nodes.is_red(self.nodes.left(self.root)) } {
            self.nodes.colors[self.root as usize] = Color::Red;
        }

        let (root, removed) = self.delete(self.root, key);
        self.root = root;
        if root < SENTINEL {
            self.nodes.colors[root as usize] = Color::Black
        }

        if removed.is_some() {
            self.len -= 1;
        }

        removed
    }
}

impl<K: Ord, V> SOARBTree<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            root: SENTINEL,
            len: 0,
        }
    }

    fn get_node(&self, mut node: u32, key: &K) -> Option<&V> {
        while node < SENTINEL {
            match key.cmp(&self.nodes.key(node)) {
                Ordering::Less => node = self.nodes.left(node),
                Ordering::Greater => node = self.nodes.right(node),
                Ordering::Equal => return Some(&self.nodes.value(node)),
            }
        }
        None
    }

    fn insert_node(
        &mut self,
        mut node: u32,
        key: K,
        value: V,
        added: &mut bool,
    ) -> (u32, Option<V>) {
        if node == SENTINEL {
            *added = true;
            return (self.nodes.add(key, value), None);
        }

        let replaced_value;
        match key.cmp(self.nodes.key(node)) {
            Ordering::Less => {
                let (left, old_value) = self.insert_node(self.nodes.left(node), key, value, added);
                self.nodes.lefts[node as usize] = left;
                replaced_value = old_value;
            }
            Ordering::Greater => {
                let (right, old_value) =
                    self.insert_node(self.nodes.right(node), key, value, added);
                self.nodes.rights[node as usize] = right;
                replaced_value = old_value;
            }
            Ordering::Equal => {
                replaced_value = Some(self.nodes.revalue(node, value));
            }
        }

        if self.nodes.is_red(self.nodes.right(node)) && !self.nodes.is_red(self.nodes.left(node)) {
            node = self.rotate_left(node);
        }
        if (self.nodes.is_red(self.nodes.left(node)))
            && self.nodes.is_red(self.nodes.left(self.nodes.left(node)))
        {
            node = self.rotate_right(node);
        }
        if (self.nodes.is_red(self.nodes.left(node))) && self.nodes.is_red(self.nodes.right(node)) {
            self.flip_colors(node);
        }

        (node, replaced_value)
    }

    fn delete(&mut self, mut node: u32, key: &K) -> (u32, Option<V>) {
        if node == SENTINEL {
            return (SENTINEL, None);
        }

        let removed;

        if key.cmp(self.nodes.key(node)) == Ordering::Less {
            if self.nodes.left(node) < SENTINEL
                && !self.nodes.is_red(self.nodes.left(node))
                && !self.nodes.is_red(self.nodes.left(self.nodes.left(node)))
            {
                node = self.move_red_left(node);
            }
            let (left, old_value) = self.delete(self.nodes.left(node), key);
            self.nodes.lefts[node as usize] = left;
            removed = old_value;
        } else {
            if self.nodes.is_red(self.nodes.left(node)) {
                node = self.rotate_right(node);
            }

            if key.cmp(self.nodes.key(node)) == Ordering::Equal
                && self.nodes.right(node) == SENTINEL
            {
                let (_, v) = self.nodes.remove(node);
                return (SENTINEL, Some(v));
            }

            if self.nodes.right(node) < SENTINEL
                && !self.nodes.is_red(self.nodes.right(node))
                && !self.nodes.is_red(self.nodes.left(self.nodes.right(node)))
            {
                node = self.move_red_right(node);
            }

            if key.cmp(self.nodes.key(node)) == Ordering::Equal {
                let (right, min) = self.pop_min(self.nodes.right(node));
                let (k, v) = self.nodes.remove(min);
                // I suppose this was easier than redirecting the parent...
                self.nodes.rekey(node, k);
                removed = Some(self.nodes.revalue(node, v));
                self.nodes.rights[node as usize] = right;
            } else {
                let (right, old_value) = self.delete(self.nodes.right(node), key);
                self.nodes.rights[node as usize] = right;
                removed = old_value;
            }
        }

        (self.balance(node), removed)
    }

    fn pop_min(&mut self, mut h: u32) -> (u32, u32) {
        if self.nodes.left(h) == SENTINEL {
            let right = self.nodes.right(h);
            self.nodes.rights[h as usize] = SENTINEL;
            return (right, h);
        }

        if !self.nodes.is_red(self.nodes.left(h))
            && !self.nodes.is_red(self.nodes.left(self.nodes.left(h)))
        {
            h = self.move_red_left(h);
        }

        let (left, min) = self.pop_min(self.nodes.left(h));
        self.nodes.lefts[h as usize] = left;
        (self.balance(h), min)
    }

    fn rotate_left(&mut self, h: u32) -> u32 {
        let x = self.nodes.right(h);
        self.nodes.rights[h as usize] = self.nodes.lefts[x as usize];
        self.nodes.lefts[x as usize] = h;
        let y = self.nodes.left(x);
        if self.nodes.is_red(y) {
            self.nodes.colors[x as usize] = Color::Red;
        } else {
            self.nodes.colors[x as usize] = Color::Black;
            self.nodes.colors[y as usize] = Color::Red;
        }
        x
    }

    fn rotate_right(&mut self, h: u32) -> u32 {
        let x = self.nodes.left(h);
        self.nodes.lefts[h as usize] = self.nodes.rights[x as usize];
        self.nodes.rights[x as usize] = h;
        let y = self.nodes.right(x);
        if self.nodes.is_red(y) {
            self.nodes.colors[x as usize] = Color::Red;
        } else {
            self.nodes.colors[x as usize] = Color::Black;
            self.nodes.colors[y as usize] = Color::Red;
        }
        x
    }

    fn flip_colors(&mut self, h: u32) {
        self.nodes.flip(h);
        let left = self.nodes.left(h);
        if left < SENTINEL {
            self.nodes.flip(left);
        }
        let right = self.nodes.right(h);
        if right < SENTINEL {
            self.nodes.flip(right);
        }
    }

    fn move_red_left(&mut self, mut h: u32) -> u32 {
        self.flip_colors(h);
        if self.nodes.is_red(self.nodes.left(self.nodes.right(h))) {
            self.nodes.rights[h as usize] = self.rotate_right(self.nodes.rights[h as usize]);
            h = self.rotate_left(h);
            self.flip_colors(h);
        }
        h
    }

    fn move_red_right(&mut self, mut h: u32) -> u32 {
        self.flip_colors(h);
        if self.nodes.is_red(self.nodes.left(self.nodes.left(h))) {
            h = self.rotate_right(h);
            self.flip_colors(h);
        }
        h
    }

    fn balance(&mut self, mut h: u32) -> u32 {
        if self.nodes.is_red(self.nodes.right(h)) {
            h = self.rotate_left(h);
        }
        if self.nodes.is_red(self.nodes.left(h))
            && self.nodes.is_red(self.nodes.left(self.nodes.left(h)))
        {
            h = self.rotate_right(h);
        }
        if self.nodes.is_red(self.nodes.left(h)) && self.nodes.is_red(self.nodes.right(h)) {
            self.flip_colors(h);
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, SOARBTree};

    #[test]
    fn tree_operations_work() {
        let mut tree = SOARBTree::new();
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
