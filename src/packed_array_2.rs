use crate::Map;
use std::cmp::Ordering;

// no bookkeeping of densities,
// just try to
pub struct PackedArray2<K, V> {
    // Sorted vector of (key, value) pairs with gaps (Some) and empty slots (None)
    data: Vec<Option<(K, V)>>,
    _len: usize,
}

impl<K: Ord, V> PackedArray2<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _len: 0,
        }
    }

    fn find(&self, key: &K) -> (bool, usize) {
        let mut start = 0;
        let mut end = self.data.len();
        'outer: while end > start {
            let mid = (start + end) / 2;
            for i in mid..end {
                if let Some((k, _)) = &self.data[i] {
                    match &key.cmp(k) {
                        Ordering::Less => {
                            end = mid;
                            continue 'outer;
                        }
                        Ordering::Equal => return (true, i),
                        Ordering::Greater => {
                            start = i + 1;
                            continue 'outer;
                        }
                    }
                }
            }
            // go left
            end = mid;
        }
        (false, start)
    }

    fn fix_insert(&mut self, from: usize, to: usize) {
        self._len += 1;
        if self._len <= 8 {
            return;
        }
        let len = (to - from + 1).next_power_of_two();
        if len > 8 {
            let start = !(len - 1) & to;
            self.balance(start, start + len)
        }
    }

    fn fix_remove(&mut self, index: usize) {
        self._len -= 1;
        if self._len * 4 < self.data.len() {
            self.condense();
            return;
        }
        let mut j = index + 1;
        while j < self.data.len() && self.data[j].is_none() {
            j += 1;
        }
        if j >= self.data.len() {
            self.data.truncate(index);
            return;
        }
        let len = (j - index).next_power_of_two();
        if len > 8 {
            let start = !(len - 1) & index;
            self.balance(start, start + len)
        }
    }

    fn balance(&mut self, start: usize, end: usize) {
        if end >= self.data.len() {
            self.data.resize_with(end + 1, || None);
        }
        // just do it twice in both directions!
        let mut count = 0;
        // first round take care of elements that come to late
        for source in start..end {
            if self.data[source].is_none() {
                continue;
            }
            let target = start + count * self.data.len() / self._len;
            if source > target {
                self.data[target] = self.data[source].take();
            }
            count += 1;
        }
        // second takes care of those to come too soon
        for source in (start..end).rev() {
            if count == 0 {
                return;
            }
            count -= 1;
            if self.data[source].is_none() {
                continue;
            }
            let target = start + count * self.data.len() / self._len;
            if source < target {
                self.data[target] = self.data[source].take();
            }
        }
        assert_eq!(count, 0);
    }

    fn condense(&mut self) {
        let mut count = 0;
        // first round takes care of elements that come to late
        let new_len = self.data.len() / 2;
        for source in 0..self.data.len() {
            if self.data[source].is_none() {
                continue;
            }
            let target = count * new_len / self._len;
            if source > target {
                self.data[target] = self.data[source].take();
            }
            count += 1;
        }
        assert_eq!(count, self._len);
        self.data.truncate(new_len);
    }
}

impl<K: Ord, V> Map<K, V> for PackedArray2<K, V> {
    fn len(&self) -> usize {
        self._len
    }

    fn is_empty(&self) -> bool {
        self._len == 0
    }

    fn contains_key(&self, key: &K) -> bool {
        self.find(key).0
    }

    fn get(&self, key: &K) -> Option<&V> {
        let (found, idx) = self.find(&key);
        if found {
            self.data[idx].as_ref().map(|(_, v)| v)
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (found, index) = self.find(&key);
        let mut pair = (key, value);
        if found {
            return self.data[index].replace(pair).map(|(_, v)| v);
        }
        // not found also means the right slot is occupied.
        for i in index..self.data.len() {
            match self.data[i].replace(pair) {
                Some(p) => pair = p,
                None => {
                    self.fix_insert(index, i);
                    return None;
                }
            }
        }
        self.data.push(Some(pair));
        self.fix_insert(index, self.data.len() - 1);
        return None;
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let (found, idx) = self.find(key);
        if !found {
            return None;
        }
        let result = self.data[idx].take().map(|(_, v)| v);
        self.fix_remove(idx);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, PackedArray2};

    #[test]
    fn test_operations_work() {
        let mut array = PackedArray2::new();
        assert!(array.is_empty());

        assert_eq!(array.insert(3, "three"), None);
        assert_eq!(array.insert(1, "one"), None);
        assert_eq!(array.insert(4, "four"), None);
        assert_eq!(array.insert(3, "tres"), Some("three"));

        assert_eq!(array.len(), 3);
        assert!(array.contains_key(&1));
        assert_eq!(array.get(&3), Some(&"tres"));
        assert_eq!(array.get(&2), None);

        log_array(&array);
        assert_eq!(array.remove(&1), Some("one"));
        log_array(&array);
        assert_eq!(array.remove(&1), None);
        log_array(&array);
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(&3), Some(&"tres"));
        log_array(&array);
        assert_eq!(array.remove(&3), Some("tres"));
        log_array(&array);
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn test_add_many() {
        let keys: Vec<i32> = (0..500 as i32).collect::<Vec<i32>>().repeat(2);
        let mut array = PackedArray2::new();
        for key in keys {
            array.insert(key, key);
        }
        assert_eq!(array.len(), 500);
    }

    #[test]
    fn test_add_many_reverse() {
        let keys: Vec<i32> = (0..500 as i32).rev().collect::<Vec<i32>>().repeat(2);
        let mut array = PackedArray2::new();
        for key in keys {
            array.insert(key, key);
        }
        assert_eq!(array.len(), 500);
    }

    #[test]
    fn test_add_and_remove_many() {
        let keys: Vec<i32> = (0..64 as i32)
            .map(|i| i.reverse_bits())
            .collect::<Vec<i32>>();
        let mut array = PackedArray2::new();
        for &key in &keys {
            array.insert(key, key);
        }
        for &key in &keys {
            log_array(&array);
            assert_eq!(array.remove(&key), Some(key));
        }
        assert_eq!(array.len(), 0);
    }

    fn log_array<K, V>(array: &PackedArray2<K, V>) {
        println!(
            "data: {}",
            (0..array.data.len())
                .map(|i| if array.data[i].is_some() { "x" } else { "_" })
                .collect::<String>()
        );
    }

    #[test]
    fn test_ordering() {
        let mut array = PackedArray2::new();
        let keys = vec![5, 2, 8, 1, 9, 3];
        for &key in &keys {
            array.insert(key, key);
        }
        assert_eq!(array.len(), keys.len());
        let mut output = Vec::new();
        for i in 0..10 {
            if let Some(&j) = array.get(&i) {
                output.push(j);
            }
        }
        assert_eq!(output, vec![1, 2, 3, 5, 8, 9]);
    }
}
