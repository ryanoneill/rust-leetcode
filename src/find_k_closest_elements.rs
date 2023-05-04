use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct State {
    distance: usize,
    num: i32,
}

impl State {
    fn new(num: i32, x: i32) -> Self {
        let distance = i32::abs(num - x) as usize;
        State { distance, num }
    }
}

/// Given a sorted integer array `arr`, two integers `k` and `x`, return the
/// `k` closest integers to `x` in the array. The result should also be sorted
/// in ascending order.
///
/// An integer `a` is closer to `x` than an integer `b` if:
/// * `|a - x| < |b - x|`, or
/// * `|a - x| == |b - x|` and `a < b`
struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut min_heap = BinaryHeap::new();
        for num in arr {
            let state = State::new(num, x);
            min_heap.push(Reverse(state));
        }
        let mut result = Vec::new();
        for _ in 0..k as usize {
            let item = min_heap.pop().unwrap().0;
            result.push(item.num);
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let result = Solution::find_closest_elements(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let result = Solution::find_closest_elements(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
