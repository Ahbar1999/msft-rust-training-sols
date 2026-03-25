pub mod generic_cache {
    use std::hash::Hash;
    use std::collections::HashMap;
    use std::collections::VecDeque;

    pub struct Cache<K, V> 
        where K: Eq + Hash + Clone {
        map: HashMap<K, V>,
        key_queue: VecDeque<K>,  // change this to Vec<&K> but since Clone is available on K we just
                            // clone it for now
        capacity: usize,
    }

    impl<K, V> Cache<K, V> 
        where K: Eq + Hash + Clone {
        pub fn new(capacity: usize) -> Self {
            Cache {
                map: HashMap::with_capacity(capacity),
                key_queue: VecDeque::with_capacity(capacity),
                capacity,
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            if !self.map.contains_key(&key) {
                self.key_queue.push_back(key.clone());
            }

            self.map.insert(key, value);

            if self.key_queue.len() == self.capacity + 1 {
                let out_key = self.key_queue.pop_front().unwrap();
                self.map.remove_entry(&out_key);
            }
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.map.get(key) 
        }

        pub fn len(&self) -> usize {
            self.key_queue.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generic_cache::*;

    // 1. Testing with different types: String and Vec
    #[test]
    fn test_complex_types() {
        let mut cache: Cache<String, Vec<i32>> = Cache::new(2);
        let key1 = String::from("numbers");
        let mut val1 = Vec::new();
        val1.push(1);
        val1.push(2);
        val1.push(3);
        
        cache.insert(key1.clone(), val1.clone());
        assert_eq!(cache.get(&key1), Some(&val1));
    }

    // 2. Testing with Custom Structs
    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    struct User { id: u32 }

    #[test]
    fn test_custom_struct_keys() {
        let mut cache = Cache::new(1);
        let u1 = User { id: 101 };
        
        cache.insert(u1.clone(), "Active");
        assert_eq!(cache.get(&u1), Some(&"Active"));
        
        // Eviction of custom struct
        cache.insert(User { id: 102 }, "Offline");
        assert_eq!(cache.get(&u1), None);
    }

    // 3. Corner Case: Re-inserting the same key (FIFO Position Check)
    #[test]
    fn test_reinsertion_does_not_reset_fifo_priority() {
        let mut cache = Cache::new(2);
        cache.insert("A", 1); // Oldest
        cache.insert("B", 2); // Newest
        
        // Update "A". In strict FIFO, "A" is still the oldest.
        cache.insert("A", 100); 
        
        assert_eq!(cache.len(), 2);
        // Insert "C", which triggers eviction.
        // If "A" is still oldest, it should be gone.
        cache.insert("C", 3);

        assert_eq!(cache.get(&"B"), Some(&2));
        assert_eq!(cache.get(&"C"), Some(&3));
        assert_eq!(cache.get(&"A"), None, "A should have been evicted as the oldest entry");
    }

    // 4. Corner Case: Capacity of 1
    #[test]
    fn test_capacity_one() {
        let mut cache = Cache::new(1);
        cache.insert("apple", 1);
        cache.insert("banana", 2); // Should immediately evict apple

        assert_eq!(cache.get(&"apple"), None);
        assert_eq!(cache.get(&"banana"), Some(&2));
        assert_eq!(cache.len(), 1);
    }

    // 5. Corner Case: Multiple evictions in a row
    #[test]
    fn test_rapid_eviction() {
        let mut cache = Cache::new(3);
        for i in 0..10 {
            cache.insert(i, i * 10);
        }
        
        assert_eq!(cache.len(), 3);
        // Only the last 3 (7, 8, 9) should remain
        assert_eq!(cache.get(&6), None);
        assert_eq!(cache.get(&7), Some(&70));
        assert_eq!(cache.get(&9), Some(&90));
    }

    // 6. Testing 'get' does not affect eviction order (Unlike LRU)
    #[test]
    fn test_get_does_not_change_order() {
        let mut cache = Cache::<usize, char>::new(2);
        cache.insert(1, 'a');
        cache.insert(2, 'b');

        // Accessing 1
        let _ = cache.get(&1);

        // Inserting 3 should still evict 1 because it was inserted first
        cache.insert(3,'c');

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&3), Some(&'c'));
    }
}

fn main() {
    // println!("Hello, world!");
}
