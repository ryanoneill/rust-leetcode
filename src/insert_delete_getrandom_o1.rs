use rand::prelude::*;
use std::collections::HashMap;

/// Implement the `RandomizedSet` class:
///
/// * `RandomizedSet()` Initializes the `RandomizedSet` object.
///
/// * `bool insert(int val)` Inserts an item `val` into the set if not present.
///   Returns `true` if the item was not present, `false` otherwise.
///
/// * `bool remove(int val)` Removes an item `val` from the set if present.
///   Returns `true` if the item was present, `false` otherwise.
///
/// * `int getRandom()` Returns a random element from the current set of
///   elements (it's guaranteed that at least one element exists when the
///   method is called). Each element must have the same probability of being
///   returned.
///
/// You must implement the functions of the class such that each function works
/// in average `O(1)` time complexity.
struct RandomizedSet {
    items: Vec<i32>,
    indices: HashMap<i32, usize>
}

impl RandomizedSet {

    fn new() -> Self {
        Self { items: Vec::new(), indices: HashMap::new() }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.indices.contains_key(&val) {
            let index = self.items.len();
            self.items.push(val);
            self.indices.insert(val, index);
            true
        } else { false }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            let n = self.items.len();
            if n == 1 {
                self.indices.clear();
                self.items.clear();
            } else {
                let index = self.indices[&val];
                if index == n-1 {
                    self.items.pop();
                    self.indices.remove(&val);
                } else {
                    self.items.swap_remove(index);
                    self.indices.remove(&val);
                    let value_at_index = self.items[index];
                    self.indices.insert(value_at_index, index);
                }
            }
            true
        } else { false }
    }

    fn get_random(&self) -> i32 {
        let n = self.items.len();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0,n);
        self.items[index]
    }

}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;

    #[test]
    fn example_1() {
        let mut set = RandomizedSet::new();
        let result = set.insert(1);
        assert!(result);
        let result = set.remove(2);
        assert!(!result);
        let result = set.insert(2);
        assert!(result);
        let result = set.get_random();
        assert!(result == 1 || result == 2);
        let result = set.remove(1);
        assert!(result);
        let result = set.insert(2);
        assert!(!result);
        let result = set.get_random();
        assert_eq!(result, 2);
    }

    #[test]
    fn real_world_1() {
        let mut set = RandomizedSet::new();
        let result = set.remove(0);
        assert!(!result);
        let result = set.remove(0);
        assert!(!result);
        let result = set.insert(0);
        assert!(result);
        let result = set.get_random();
        assert_eq!(result, 0);
        let result = set.remove(0);
        assert!(result);
        let result = set.insert(0);
        assert!(result);
    }

}
