//! Least Recently Used Map

/// The LruMap (i.e. Least Recently Used Map) is a map of keys to values, with a fixed capacity.
/// Adding keys beyond the capacity will remove the least recently accessed key-value tuple and return it.
pub struct LruMap<K: PartialEq, V, const S: usize> {
    data: arrayvec::ArrayVec<(usize, K, V), S>,
    next_operation: usize,
}

impl<K: PartialEq, V, const S: usize> LruMap<K, V, S> {
    /// Create a new LruMap
    pub const fn new() -> Self {
        LruMap {
            data: arrayvec::ArrayVec::new_const(),
            next_operation: 0,
        }
    }

    /// Insert a new entry to the cache, and evict the least recently used one if capacity has been reached
    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)> {
        let mut popped = None;
        let new_op = self.get_and_inc_op();
        if self.data.is_full() {
            let lru = self.least_recently_used().expect(
                "Capacity was full and LRU was not found. Confirm LRU Map capacity is not zero?",
            );
            let (_op, k, v) = self.data.remove(lru);
            popped = Some((k, v));
        }
        self.data.push((new_op, key, value));
        popped
    }

    /// Get the value by key if it exists
    ///
    /// If you need a mutable reference, you can use "as_mut"
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // TODO what if max is reached? Need to rebalance all entries
        let new_op = self.get_and_inc_op();
        for (op, k, v) in self.data.iter_mut() {
            if key == k {
                *op = new_op;
                return Some(v);
            }
        }
        None
    }

    /// Returns None if there is still more capacity, or if there is no LRU.
    pub fn get_least_recently_used(&mut self) -> Option<(&mut K, &mut V)> {
        if !self.data.is_full() {
            return None;
        };
        match self.least_recently_used() {
            None => None,
            Some(index) => {
                let new_op = self.get_and_inc_op();
                let (op, k, v) = self.data.get_mut(index)?;
                *op = new_op;
                Some((k, v))
            }
        }
    }

    /// Returns the capacity of the map
    pub fn capacity(&self) -> usize {
        S
    }

    /// Returns the len of the map. Can be used to determine if you should use insert or get_least_recently_used
    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn get_and_inc_op(&mut self) -> usize {
        let v = self.next_operation;
        self.next_operation += 1;
        v
    }

    fn least_recently_used(&self) -> Option<usize> {
        struct IndexAndOp {
            index: usize,
            operation: usize,
        }
        let mut ret_least_index: Option<IndexAndOp> = None;
        for (i, (sz, _k, _v)) in self.data.iter().enumerate() {
            match &mut ret_least_index {
                None => {
                    ret_least_index = Some(IndexAndOp {
                        index: i,
                        operation: *sz,
                    });
                }
                Some(least) => {
                    if &least.operation > sz {
                        ret_least_index = Some(IndexAndOp {
                            index: i,
                            operation: *sz,
                        });
                    }
                }
            }
        }
        ret_least_index.map(|index_and_op| index_and_op.index)
    }
}

#[cfg(test)]
mod test {
    use crate::structs::lru_map::LruMap;

    #[test]
    pub fn can_add_and_remove_lru() {
        let mut lru: LruMap<_, _, 2> = LruMap::new();
        assert!(lru.insert(1, "one").is_none());
        assert!(lru.insert(2, "two").is_none());
        let evicted = lru.insert(3, "three").unwrap();
        assert_eq!(evicted, (1, "one"));
        // Increase the recency when used
        assert_eq!(lru.get(&2), Some(&"two"));
        let evicted = lru.insert(4, "four").unwrap();
        assert_eq!(evicted, (3, "three"));
    }
}
