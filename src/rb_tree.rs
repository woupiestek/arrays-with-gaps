use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    color: Color,
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            color: Color::Red,
            key,
            value,
            left: None,
            right: None,
        }
    }
}

pub struct RedBlackTree<K, V> {
    root: Link<K, V>,
    len: usize,
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_node(&self.root, key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut added = false;
        let (root, old_value) = Self::insert_node(self.root.take(), key, value, &mut added);
        self.root = root;
        if let Some(ref mut root_node) = self.root {
            root_node.color = Color::Black;
        }
        if added {
            self.len += 1;
        }
        old_value
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.root.is_none() {
            return None;
        }

        if !Self::is_red(&self.root) && !Self::is_red(&self.root.as_ref().unwrap().left) {
            if let Some(ref mut root_node) = self.root {
                root_node.color = Color::Red;
            }
        }

        let (root, removed) = Self::delete(self.root.take(), key);
        if let Some(mut new_root) = root {
            new_root.color = Color::Black;
            self.root = Some(new_root);
        } else {
            self.root = None;
        }

        if removed.is_some() {
            self.len -= 1;
        }

        removed
    }

    fn get_node<'a>(node: &'a Link<K, V>, key: &K) -> Option<&'a V> {
        let mut current = node.as_ref();
        while let Some(node) = current {
            match key.cmp(&node.key) {
                Ordering::Less => current = node.left.as_ref(),
                Ordering::Greater => current = node.right.as_ref(),
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }

    fn insert_node(
        node: Link<K, V>,
        key: K,
        value: V,
        added: &mut bool,
    ) -> (Link<K, V>, Option<V>) {
        let mut h = match node {
            Some(node) => node,
            None => {
                *added = true;
                return (Some(Box::new(Node::new(key, value))), None);
            }
        };

        let replaced_value;
        match key.cmp(&h.key) {
            Ordering::Less => {
                let (left, old_value) = Self::insert_node(h.left.take(), key, value, added);
                h.left = left;
                replaced_value = old_value;
            }
            Ordering::Greater => {
                let (right, old_value) = Self::insert_node(h.right.take(), key, value, added);
                h.right = right;
                replaced_value = old_value;
            }
            Ordering::Equal => {
                replaced_value = Some(std::mem::replace(&mut h.value, value));
            }
        }

        if Self::is_red(&h.right) && !Self::is_red(&h.left) {
            h = Self::rotate_left(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.right) {
            Self::flip_colors(&mut h);
        }

        (Some(h), replaced_value)
    }

    fn delete(node: Link<K, V>, key: &K) -> (Link<K, V>, Option<V>) {
        let mut h = match node {
            Some(node) => node,
            None => return (None, None),
        };

        let removed;

        if key.cmp(&h.key) == Ordering::Less {
            if h.left.is_some()
                && !Self::is_red(&h.left)
                && !Self::is_red(&h.left.as_ref().unwrap().left)
            {
                h = Self::move_red_left(h);
            }
            let (left, old_value) = Self::delete(h.left.take(), key);
            h.left = left;
            removed = old_value;
        } else {
            if Self::is_red(&h.left) {
                h = Self::rotate_right(h);
            }

            if key.cmp(&h.key) == Ordering::Equal && h.right.is_none() {
                return (None, Some(h.value));
            }

            if h.right.is_some()
                && !Self::is_red(&h.right)
                && !Self::is_red(&h.right.as_ref().unwrap().left)
            {
                h = Self::move_red_right(h);
            }

            if key.cmp(&h.key) == Ordering::Equal {
                let (right, min) = Self::pop_min(h.right.take().unwrap());
                let removed_value = Some(h.value);
                h.key = min.key;
                h.value = min.value;
                h.right = right;
                removed = removed_value;
            } else {
                let (right, old_value) = Self::delete(h.right.take(), key);
                h.right = right;
                removed = old_value;
            }
        }

        (Some(Self::balance(h)), removed)
    }

    fn pop_min(mut h: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
        if h.left.is_none() {
            return (h.right.take(), h);
        }

        if !Self::is_red(&h.left) && !Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::move_red_left(h);
        }

        let (left, min) = Self::pop_min(h.left.take().unwrap());
        h.left = left;
        (Some(Self::balance(h)), min)
    }

    fn is_red(node: &Link<K, V>) -> bool {
        node.as_ref().map_or(false, |node| node.color == Color::Red)
    }

    fn rotate_left(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.right.take().unwrap();
        h.right = x.left.take();
        x.left = Some(h);
        x.color = x.left.as_ref().unwrap().color;
        x.left.as_mut().unwrap().color = Color::Red;
        x
    }

    fn rotate_right(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.left.take().unwrap();
        h.left = x.right.take();
        x.right = Some(h);
        x.color = x.right.as_ref().unwrap().color;
        x.right.as_mut().unwrap().color = Color::Red;
        x
    }

    fn flip_colors(h: &mut Box<Node<K, V>>) {
        h.color = match h.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        if let Some(ref mut left) = h.left {
            left.color = match left.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }

        if let Some(ref mut right) = h.right {
            right.color = match right.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
    }

    fn move_red_left(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Self::flip_colors(&mut h);
        if Self::is_red(&h.right.as_ref().unwrap().left) {
            h.right = Some(Self::rotate_right(h.right.take().unwrap()));
            h = Self::rotate_left(h);
            Self::flip_colors(&mut h);
        }
        h
    }

    fn move_red_right(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Self::flip_colors(&mut h);
        if Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
            Self::flip_colors(&mut h);
        }
        h
    }

    fn balance(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Self::is_red(&h.right) {
            h = Self::rotate_left(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.right) {
            Self::flip_colors(&mut h);
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::RedBlackTree;

    #[test]
    fn tree_operations_work() {
        let mut tree = RedBlackTree::new();
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
