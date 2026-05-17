use crate::Map;
use std::cmp::Ordering;

// // make some choices asshole!
// // and stick with them until the bench makrs are runnign again.

// keep track of shares of segments used by the tree,
// shared are tracked as x/120, because is it the multiple of 8 less than half of 256
struct SegmentTree {
    len: usize,
    shares: Box<[u8]>,
}

impl SegmentTree {
    const FULL: u8 = 120;

    fn new() -> Self {
        Self {
            len: 4,
            shares: vec![0; 7].into_boxed_slice(),
        }
    }

    fn _grow(&mut self, new_len: usize) {
        // I don't want to try something else.
        assert!(new_len.is_power_of_two());
        let mut new_shares = vec![0; new_len * 2 - 1].into_boxed_slice();
        let mut len = self.len;
        let mut offset = 0;
        let mut new_offset = 0;
        loop {
            new_shares[new_offset..(new_offset + len)]
                .copy_from_slice(&self.shares[offset..(offset + len)]);
            len /= 2;
            if len == 0 {
                break;
            }
            offset = offset / 2 + self.len;
            new_offset = new_offset / 2 + new_len;
        }
        self.len = new_len;
        self.shares = new_shares;
    }

    // set range...
    fn set(&mut self, index: usize, value: u8) {
        assert!(
            value <= Self::FULL,
            "value {} out of range at index {}",
            value,
            index
        );
        if index >= self.len {
            self._grow((index + 1).next_power_of_two());
        }
        self.shares[index] = value;
        let mut previous = index;
        loop {
            let current = previous / 2 + self.len;
            if current > (self.len - 1) * 2 {
                return;
            }
            self.shares[current] = (self.shares[previous] + self.shares[previous ^ 1] + 1) / 2;
            previous = current
        }
    }

    fn incr(&mut self, index: usize) -> u8 {
        let value = self.get(index) + 15;
        self.set(index, value);
        value
    }

    fn decr(&mut self, index: usize) -> u8 {
        let value = self.get(index) - 15;
        self.set(index, value);
        value
    }

    fn get(&self, index: usize) -> u8 {
        if index >= self.len {
            0
        } else {
            self.shares[index]
        }
    }

    // 2 ** power sized range needed to get everything under threshold.
    fn power_below(&self, index: usize, threshold: u8) -> (u8, u8) {
        let mut power = 0;
        let mut i = index;
        loop {
            let share = self.shares[i];
            if self.shares[i] <= threshold {
                return (power, share);
            }
            power += 1;
            i = i / 2 + self.len;
            if i > (self.len - 1) * 2 {
                return (power, share / 2);
            }
        }
    }

    // 2 ** power sized range needed to get over the threshold, if possible!
    fn power_above(&self, index: usize, threshold: u8) -> Option<(u8, u8)> {
        let mut power = 0;
        let mut i = index;
        loop {
            let share = self.shares[i];
            if self.shares[i] >= threshold {
                return Some((power, share));
            }
            power += 1;
            i = i / 2 + self.len;
            if i > (self.len - 1) * 2 {
                return None;
            }
        }
    }
}

// let's debug the part that should function without rebalancing.
pub struct PackedArray<K, V> {
    // Sorted vector of (key, value) pairs with gaps (Some) and empty slots (None)
    data: Vec<Option<(K, V)>>,
    segments: SegmentTree,
    _len: usize,
}

impl<K: Ord, V> PackedArray<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            segments: SegmentTree::new(),
            _len: 0,
        }
    }

    fn find(&self, key: &K) -> (bool, usize) {
        let mut start = 0;
        let mut end = self.data.len();
        // should this be strictly less than?
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
            // all empty, so just go left
            end = mid;
        }
        (false, start)
    }

    fn fix_insert(&mut self, index: usize) {
        self._len += 1;
        self.segments.incr(index / 8);
        if self._len <= 1 {
            return;
        }
        if self.segments.get(index / 8) < SegmentTree::FULL {
            return;
        }
        let (power, share) = self.segments.power_below(index / 8, 105);
        let len = 1 << (power + 3);
        let start = !(len - 1) & index;
        self.balance(share, start, start + len);
    }

    fn fix_remove(&mut self, index: usize) {
        self._len -= 1;
        self.segments.decr(index / 8);
        if self.segments.get(index / 8) > 60 || self._len < 4 {
            return;
        }
        if let Some((power, share)) = self.segments.power_above(index / 8, 52) {
            let len = 1 << (power + 3);
            let start = !(len - 1) & index;
            self.balance(share, start, start + len);
        } else {
            self.condense();
        }
    }

    // disappointing...
    fn balance(&mut self, share: u8, start: usize, end: usize) {
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
            let target = start + count * 120 / share as usize;
            if source > target {
                // assert!(self.data[target].is_none());
                self.data[target] = self.data[source].take();
                self.segments.decr(source / 8);
                self.segments.incr(target / 8);
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
            let target = start + count * 120 / share as usize;
            if source < target {
                // assert!(self.data[target].is_none());
                self.data[target] = self.data[source].take();
                self.segments.decr(source / 8);
                self.segments.incr(target / 8);
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
                // assert!(self.data[target].is_none());
                self.data[target] = self.data[source].take();
                self.segments.decr(source / 8);
                self.segments.incr(target / 8);
            }
            count += 1;
        }
        assert_eq!(count, self._len);
        self.data.truncate(new_len);
    }
}

impl<K: Ord, V> Map<K, V> for PackedArray<K, V> {
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
                    self.fix_insert(i);
                    return None;
                }
            }
        }
        self.data.push(Some(pair));
        self.fix_insert(self.data.len() - 1);
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
    use super::{Map, PackedArray};

    #[test]
    fn test_operations_work() {
        let mut array = PackedArray::new();
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
        let mut array = PackedArray::new();
        for key in keys {
            array.insert(key, key);
            println!("shares: {:?}", array.segments.shares);
        }
        assert_eq!(array.len(), 500);
    }

    #[test]
    fn test_add_many_reverse() {
        let keys: Vec<i32> = (0..500 as i32).rev().collect::<Vec<i32>>().repeat(2);
        let mut array = PackedArray::new();
        for key in keys {
            array.insert(key, key);

            println!("shares: {:?}", array.segments.shares);
        }
        assert_eq!(array.len(), 500);
    }

    #[test]
    fn test_add_and_remove_many() {
        let keys: Vec<i32> = (0..64 as i32)
            .map(|i| i.reverse_bits())
            .collect::<Vec<i32>>();
        let mut array = PackedArray::new();
        for &key in &keys {
            array.insert(key, key);
        }
        for key in &keys {
            log_array(&array);
            assert!(array.remove(key).is_some());
        }
        assert_eq!(array.len(), 0);
    }

    fn log_array<K, V>(array: &PackedArray<K, V>) {
        println!(
            "data: {}",
            (0..array.data.len())
                .map(|i| if array.data[i].is_some() { "x" } else { "_" })
                .collect::<String>()
        );
    }

    #[test]
    fn test_ordering() {
        let mut array = PackedArray::new();
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
