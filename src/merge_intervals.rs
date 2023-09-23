use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {

    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    pub fn overlaps(&self, other: &Interval) -> bool {
        // Don't overlap if self.end < other.start or other.end < self.start
        !(self.end < other.start || other.end < self.start)
    }

    pub fn combine(&mut self, other: &Interval) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

}

/// Given an array of `intervals` where `intervals[i] = [starti, endi]`, merge all overlapping
/// intervals, and return an array of the non-overlapping intervals that cover all the intervals in
/// the input.
struct Solution;

impl Solution {

    fn to_min_heap(intervals: Vec<Vec<i32>>) -> BinaryHeap<Reverse<Interval>> {
        let mut results = BinaryHeap::new();

        for item in intervals {
            let interval = Interval::new(item[0], item[1]);
            results.push(Reverse(interval));
        }

        results
    }

    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut min_heap = Self::to_min_heap(intervals);

        if !min_heap.is_empty() {
            let mut current = min_heap.pop().unwrap().0;

            while !min_heap.is_empty() {
                let next = min_heap.pop().unwrap().0;
                if current.overlaps(&next) {
                    current.combine(&next);
                } else {
                    results.push(vec![current.start, current.end]);
                    current = next;
                }
            }

            results.push(vec![current.start, current.end]);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]];
        let result = Solution::merge(intervals);
        assert_eq!(result, vec![vec![1,6], vec![8,10], vec![15,18]]);
    }

    #[test]
    fn example_2() {
        let intervals = vec![vec![1,4], vec![4,5]];
        let result = Solution::merge(intervals);
        assert_eq!(result, vec![vec![1,5]]);
    }

    #[test]
    fn example_3() {
        let intervals = vec![vec![4,5], vec![1,4]];
        let result = Solution::merge(intervals);
        assert_eq!(result, vec![vec![1,5]]);
    }

}
