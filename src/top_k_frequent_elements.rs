use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Ord, Eq)]
struct State {
    count: usize,
    num: i32,
}

impl State {
    fn new(num: i32) -> Self {
        State { count: 1, num }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

/// Given an integer array `nums` and an integer `k`, return the `k` most
/// frequent elements. You may return the answer in any order.
struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, State> = HashMap::new();
        for num in nums {
            counts
                .entry(num)
                .and_modify(|s| {
                    s.increment();
                })
                .or_insert(State::new(num));
        }
        let mut max_heap = BinaryHeap::new();
        for count in counts {
            max_heap.push(count.1);
        }
        let mut result = Vec::new();
        for _ in 0..k as usize {
            let num = max_heap.pop().unwrap().num;
            result.push(num);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }
}
