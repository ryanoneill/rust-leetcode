use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
struct CacheNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<CacheNode>>>,
    next: Option<Rc<RefCell<CacheNode>>>,
}

impl CacheNode {

    fn new(key: i32, value: i32) -> Self {
        Self { key, value, prev: None, next: None }
    }

}

impl fmt::Debug for CacheNode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CacheNode")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("prev", &self.prev.get_key())
            .field("next", &self.next.get_key())
            .finish()
    }

}

trait CacheNodeAdditions {
    fn get_key(&self) -> Option<i32>;
    fn get_value(&self) -> Option<i32>;
    fn set_value(&self, value: i32);
    fn has_key_value(&self, key: i32, value: i32) -> bool;

    fn set_prev(&self, prev: Self);
    fn set_next(&self, next: Self);

    fn take_prev(&self) -> Self;
    fn take_next(&self) -> Self;
}

impl CacheNodeAdditions for Option<Rc<RefCell<CacheNode>>> {

    fn get_key(&self) -> Option<i32> {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                Some(node.key)
            }
            None => None
        }
    }

    fn get_value(&self) -> Option<i32> {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                Some(node.value)
            }
            None => None
        }
    }

    fn set_value(&self, value: i32) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.value = value;
            }
            None => { }
        }
    }

    fn has_key_value(&self, key: i32, value: i32) -> bool {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                node.key == key && node.value == value
            }
            None => false
        }

    }

    fn set_prev(&self, prev: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.prev = prev;
            }
            None => { } // do nothing
        }
    }

    fn set_next(&self, next: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.next = next;
            }
            None => { } // do nothing
        }
    }

    fn take_prev(&self) -> Self {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.prev.take()
            }
            None => {
                None
            }
        }
    }

    fn take_next(&self) -> Self {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.next.take()
            }
            None => {
                None
            }
        }
    }

}

trait CacheNodeCellAdditions {

    fn wrapped(self) -> Option<Rc<RefCell<Self>>>;

}

impl CacheNodeCellAdditions for CacheNode {

    fn wrapped(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }

}

// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
///
/// Implement the `LRUCache` class:
///
/// * `LRUCache(int capacity)` Initialize the LRU cache with positive size `capacity`.
///
/// * `int get(int key)` Return the value of the `key` if the key exists, otherwise return `-1`.
///
/// * `void put(int key, int value)` Update the value of the `key` if the `key` exists. Otherwise,
///   add the `key-value` pair to the cache. If the number of keys exceeds the `capacity` form this
///   operation, evict the least recently used key.
///
/// The functions `get` and `put` must each run in `O(1)` average time complexity.
struct LRUCache {
    capacity: usize,
    items: HashMap<i32, Option<Rc<RefCell<CacheNode>>>>,
    front: Option<Rc<RefCell<CacheNode>>>,
    back: Option<Rc<RefCell<CacheNode>>>,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        let front = CacheNode::new(i32::MIN, i32::MIN).wrapped();
        let back = CacheNode::new(i32::MAX, i32::MAX).wrapped();

        front.set_next(back.clone());
        back.set_prev(front.clone());

        Self {
            capacity: capacity as usize,
            items: HashMap::new(),
            front,
            back,
        }
    }

    fn push_back(&mut self, item: Option<Rc<RefCell<CacheNode>>>) {
        if item.is_some() {
            let prev = self.back.take_prev();
            let next = prev.take_next();

            prev.set_next(item.clone());
            next.set_prev(item.clone());

            item.set_prev(prev);
            item.set_next(next);
        }
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<CacheNode>>> {
        let result = self.front.take_next();
        if result == self.back {
            self.front.set_next(result);
            None
        } else {
            let result_next = result.take_next();
            let result_prev = result.take_prev();

            result_next.set_prev(result_prev);
            self.front.set_next(result_next);

            result
        }
    }

    fn remove(&mut self, item: &Option<Rc<RefCell<CacheNode>>>) {
        if item.is_some() {
            let prev = item.take_prev();
            let next = item.take_next();

            prev.set_next(next.clone());
            next.set_prev(prev.clone());
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let mut result = -1;
        if let Some(item) = self.items.get(&key) {
            result = item.get_value().unwrap();
            let cloned = item.clone();
            self.remove(&cloned);
            self.push_back(cloned);
        } 
        result
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(item) = self.items.get(&key) {
            let cloned = item.clone();
            cloned.set_value(value);
            self.remove(&cloned);
            self.push_back(cloned);
        } else {
            if self.items.len() == self.capacity {
                let old = self.pop_front();
                let key = old.get_key().unwrap();
                self.items.remove(&key);
            } 
            let item = CacheNode::new(key, value).wrapped();
            self.push_back(item.clone());
            self.items.insert(key, item);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn example_1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        let result = lru_cache.get(1);
        assert_eq!(result, 1);
        lru_cache.put(3, 3);
        let result = lru_cache.get(2);
        assert_eq!(result, -1);
        lru_cache.put(4, 4);
        let result = lru_cache.get(1);
        assert_eq!(result, -1);
        let result = lru_cache.get(3);
        assert_eq!(result, 3);
        let result = lru_cache.get(4);
        assert_eq!(result, 4);
    }

    #[test]
    fn diff_key_value() {
        let mut lru_cache = LRUCache::new(1);
        lru_cache.put(2, 1);
        let result = lru_cache.get(2);
        assert_eq!(result, 1);
    }

}
