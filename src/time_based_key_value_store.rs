use std::collections::HashMap;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Entry {
    timestamp: i32,
    value: String,
}

impl Entry {

    fn new(timestamp: i32, value: String) -> Self {
        Self { timestamp, value }
    }

}

/// Design a time-based key-value data structure that can store multiple values for the same key at
/// different time stamps and retrieve the key's value at a certain timestamp.
///
/// Implement the `TimeMap` class:
///
/// * `TimeMap()` Initializes the object of the data structure.
///
/// * `void set(String key, String value, int timestamp)` Stores the key `key` with the value
///   `value` at the given time `timestamp`.
///
///
/// * `String get(String key, int timestamp)` Returns a value such that `set` was called
///   previously, with `timestamp_prev <= timestamp`. If there are multiple such values, it returns
///   the value associated with the largest `timestamp_prev`. If there are no values, it returns
///   `""`.
struct TimeMap {
    values: HashMap<String, Vec<Entry>>
}

impl TimeMap {

    fn new() -> Self {
        Self { values: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = Entry::new(timestamp, value);
        self
            .values
            .entry(key)
            .or_insert(Vec::new())
            .push(entry);
    }

    fn find_value(entries: &Vec<Entry>, timestamp: i32) -> String {
        let mut result = "";

        let mut left = 0;
        let mut right = entries.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if entries[mid].timestamp <= timestamp {
                result = &entries[mid].value;
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                } else {
                    right = mid - 1;
                }
            }
        }

        result.to_string()
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if self.values.contains_key(&key) {
            let entries: &Vec<Entry> = &self.values[&key];
            Self::find_value(entries, timestamp)
        } else {
            "".to_string()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::TimeMap;

    #[test]
    fn example_1() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        let result = time_map.get("foo".to_string(), 1);
        assert_eq!(result, "bar");
        let result = time_map.get("foo".to_string(), 3);
        assert_eq!(result, "bar");
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        let result = time_map.get("foo".to_string(), 4);
        assert_eq!(result, "bar2");
        let result = time_map.get("foo".to_string(), 5);
        assert_eq!(result, "bar2");
    }

}
