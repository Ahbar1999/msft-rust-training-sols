pub mod repository {
    pub trait Repository {
        type Id;
        type Error;
        type Item;
        
        fn insert(&mut self, key: Self::Id, value: Self::Item) -> Option<Self::Error>;
        fn get(&self, key: &Self::Id) -> Option<&Self::Item>;
        fn erase(&mut self, key: &Self::Id) -> Result<Self::Item, Self::Error>;
    }
    
    // update key with val return old value 
    pub fn update<T: Repository>(repo: &mut T, key: T::Id, val: T::Item) -> Option<T::Item> {
        let mut old_val = None;

        if repo.get(&key).is_some() {
            old_val = repo.erase(&key).ok();
            repo.insert(key, val);
        }
        
        old_val
    }
}

pub mod generic_cache {
    use std::hash::Hash;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use crate::repository::Repository;
        
    // reusing implementation from previous assignment
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
    }

    impl<K, V> Repository for Cache<K, V> where K: Eq + Hash + Clone {
        type Id = K;
        type Item = V;
        type Error = &'static str;    // message

        fn insert(&mut self, key: Self::Id, value: Self::Item) -> Option<Self::Error> {
            if self.map.contains_key(&key) {
                self.key_queue.push_back(key.clone());

                return Some("Key already exists"); 
            }

            self.map.insert(key, value);

            if self.key_queue.len() == self.capacity + 1 {
                let out_key = self.key_queue.pop_front().unwrap();
                self.map.remove_entry(&out_key);
            }

            None
        }
        
        fn get(&self, key: &Self::Id) -> Option<&Self::Item> {
            println!("get()");
            self.map.get(&key)
        }
        
        fn erase(&mut self, key: &Self::Id) -> Result<Self::Item, Self::Error> {
            if !self.map.contains_key(key) {
                return Err("Key Does not exist!");
            }

            let (_, val) = self.map.remove_entry(key).unwrap();
            
            Ok(val)
        }
    }
}

fn main() {
}

#[cfg(test)]
pub mod tests {
    use crate::generic_cache::Cache;
    use crate::repository::*;

    #[test]
    fn basic() {
        let mut repo = Cache::<i32, i32>::new(10);

        repo.insert(1, 2);
        
        assert_eq!(repo.get(&1), Some(&2));
        
        update(&mut repo, 1, 3);

        assert_eq!(repo.get(&1), Some(&3));
    }
}
