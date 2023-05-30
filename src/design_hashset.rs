/// Design a HashSet without using any built-in hash table libraries.
///
/// Implement `MyHashSet` class:
///
/// * `void add(key)` Inserts the value `key` into the HashSet.
///
/// * `bool contains(key)` Returns whether the value `key` exists in the
///   HashSet or not.
///
/// * `void remove(key)` Removes the value `key` in the HashSet. If `key` does
///   not exist in the HashSet, do nothing.
struct MyHashSet {
    // Keys are in the range 0 <= key <= 1,000,000
    keys: Vec<bool>
}

impl MyHashSet {

    fn new() -> Self {
        Self { keys: vec![false; 1000001] }
    }

    fn add(&mut self, key: i32) {
        let index = key as usize;
        self.keys[index] = true;
    }

    fn remove(&mut self, key: i32) {
        let index = key as usize;
        self.keys[index] = false;
    }

    fn contains(&self, key: i32) -> bool {
        let index = key as usize;
        self.keys[index]
    }

}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn example_1() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.add(2);
        let result = my_hash_set.contains(1);
        assert!(result);
        let result = my_hash_set.contains(3);
        assert!(!result);
        my_hash_set.add(2);
        let result = my_hash_set.contains(2);
        assert!(result);
        my_hash_set.remove(2);
        let result = my_hash_set.contains(2);
        assert!(!result);
    }

}
