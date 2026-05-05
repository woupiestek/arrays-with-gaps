pub trait Map<K: Ord, V> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn contains_key(&self, key: &K) -> bool;

    fn get(&self, key: &K) -> Option<&V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;
}
