use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

/// You have a set which contains all positive integers `[1, 2, 3, 4, 5, ...]`.
///
/// Implement the `SmallestInfiniteSet` class:
/// * `SmallestInfiniteSet()` Initializes the SmallestInfiniteSet object to
///   contain all positive integers.
/// * `int popSmallest()` Removes and returns the smallest integer contained
///   in the infinite set.
/// * `void addBack(int num)` Adds a positive integer `num` back into the
///   infinite set, if it is not already in the infinite set.
struct SmallestInfiniteSet {
    items: HashSet<i32>,
    heap: BinaryHeap<Reverse<i32>>
}

impl SmallestInfiniteSet {

    fn new() -> Self {
        let mut result = SmallestInfiniteSet {
            items: HashSet::with_capacity(1000),
            heap: BinaryHeap::with_capacity(1000)
        };
        for i in 1..=1000 {
            result.add_back(i as i32);
        }
        result
    }

    fn pop_smallest(&mut self) -> i32 {
        let smallest = self.heap.pop().unwrap();
        let value = smallest.0;
        self.items.remove(&value);
        value
    }

    fn add_back(&mut self, num: i32) {
        if !self.items.contains(&num) {
            self.heap.push(Reverse(num));
            self.items.insert(num);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn example_1() {
        let mut smallest_infinite_set = SmallestInfiniteSet::new();
        smallest_infinite_set.add_back(2);
        let result1 = smallest_infinite_set.pop_smallest();
        assert_eq!(result1, 1);
        let result2 = smallest_infinite_set.pop_smallest();
        assert_eq!(result2, 2);
        let result3 = smallest_infinite_set.pop_smallest();
        assert_eq!(result3, 3);
        smallest_infinite_set.add_back(1);
        let result4 = smallest_infinite_set.pop_smallest();
        assert_eq!(result4, 1);
        let result5 = smallest_infinite_set.pop_smallest();
        assert_eq!(result5, 4);
        let result6 = smallest_infinite_set.pop_smallest();
        assert_eq!(result6, 5);
    }

}
