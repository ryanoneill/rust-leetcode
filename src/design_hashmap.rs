/// Design a HashMap without using any built-in hash table libraries.
///
/// Implement the `MyHashMap` class:
///
/// * `MyHashMap()` initializes the object with an empty map.
///
/// * `void put(int key, int value)` inserts a `(key, value)` pair into the HashMap. If the `key`
///   already exists in the map, update the corresponding `value`.
///
/// * `int get(int key)` returns the `value` to which the specified `key` is mapped, or `-1` if
///   this map contains no mapping for the `key`.
///
/// * `void remove(key)` removes the `key` and its corresponding `value` if the map contains the
///   mapping for the `key`.
struct MyHashMap {
    items: Vec<i32>,
}

impl MyHashMap {

    // 0 <= key, value <= 10^6
    fn new() -> Self {
        let items = vec![-1; 1000001];
        Self { items }
    }

    fn put(&mut self, key: i32, value: i32) {
        let key = key as usize;
        self.items[key] = value;
    }

    fn get(&self, key: i32) -> i32 {
        let key = key as usize;
        self.items[key]
    }

    fn remove(&mut self, key: i32) {
        let key = key as usize;
        self.items[key] = -1;
    }

}

#[cfg(test)]
mod tests {
    use super::MyHashMap;

    #[test]
    fn example_1() {
        let mut my_hash_map = MyHashMap::new();
        my_hash_map.put(1, 1);
        my_hash_map.put(2, 2);
        let result = my_hash_map.get(1);
        assert_eq!(result, 1);
        let result = my_hash_map.get(3);
        assert_eq!(result, -1);
        my_hash_map.put(2, 1);
        let result = my_hash_map.get(2);
        assert_eq!(result, 1);
        my_hash_map.remove(2);
        let result = my_hash_map.get(2);
        assert_eq!(result, -1);
    }

}
