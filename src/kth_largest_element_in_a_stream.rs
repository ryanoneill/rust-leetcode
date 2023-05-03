use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Design a class to find the `kth` largest element in a stream. Note that it
/// is the `kth` largest element in the sorted order, not the `kth` distinct
/// element.
///
/// Implement `KthLargest` class:
///
/// * `KthLargest(int k, int[] nums)` Initializes the object with the integer
///   `k` and the stream of integers `nums`.
/// * `int add(int val)` Appends the integer `val` to the stream and returns
///   the element representing the `kth` largest element in the stream.
struct KthLargest {
    k: i32,
    min_heap: BinaryHeap<Reverse<i32>>
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut result = Self { k, min_heap: BinaryHeap::new() };
        for num in nums {
            let _ = result.add(num);
        }
        result
    }

    fn peek(&self) -> i32 {
        if self.min_heap.peek().is_some() {
            self.min_heap.peek().unwrap().0
        } else { 0 }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.k as usize {
            self.min_heap.push(Reverse(val));
        } else if val > self.peek() {
            self.min_heap.push(Reverse(val));
            let _ = self.min_heap.pop();
        }
        self.peek()
    }

}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn example_1() {
        let mut kth_largest = KthLargest::new(3, vec![4,5,8,2]);
        let mut result = kth_largest.add(3);
        assert_eq!(result, 4);
        result = kth_largest.add(5);
        assert_eq!(result, 5);
        result = kth_largest.add(10);
        assert_eq!(result, 5);
        result = kth_largest.add(9);
        assert_eq!(result, 8);
        result = kth_largest.add(4);
        assert_eq!(result, 8);
    }

}
