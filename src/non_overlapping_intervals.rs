use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Interval {
    end: i32,
    start: i32,
}

impl Interval {

    pub fn new (start: i32, end: i32) -> Self {
        Self { start, end }
    }

}

/// Given an array of intervals `intervals` where `intervals[i] = [starti, endi]`, return the
/// minimum number of intervals you need to remove to make the rest of the intervals
/// non-overlapping.
struct Solution;

impl Solution {

    fn to_heap(intervals: Vec<Vec<i32>>) -> BinaryHeap<Reverse<Interval>> {
        let n = intervals.len();
        let mut result = BinaryHeap::with_capacity(n);
        for item in intervals {
            let start = item[0];
            let end = item[1];
            let interval = Interval::new(start, end);
            result.push(Reverse(interval));
        }
        result
    }

    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut heap = Self::to_heap(intervals);
        let mut result = 0;

        let mut end = i32::MIN;

        while !heap.is_empty() {
            let next = heap.pop().unwrap().0;
            if next.start < end {
                result += 1;
            } else {
                end = next.end;
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1,2], vec![2,3], vec![3,4], vec![1,3]];
        let result = Solution::erase_overlap_intervals(intervals);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let intervals = vec![vec![1,2], vec![1,2], vec![1,2]];
        let result = Solution::erase_overlap_intervals(intervals);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let intervals = vec![vec![1,2], vec![2,3]];
        let result = Solution::erase_overlap_intervals(intervals);
        assert_eq!(result, 0);
    }

}
