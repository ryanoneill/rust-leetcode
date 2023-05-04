use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
struct Meeting {
    start: i32,
    end: i32,
}

impl Meeting {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

/// Given an array of meeting time `intervals` where
/// `intervals[i] = [starti, endi]`, determine if a person could attend all
/// meetings.
struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut min_heap = BinaryHeap::new();
        for interval in intervals {
            min_heap.push(Reverse(Meeting::new(interval[0], interval[1])));
        }

        let mut attending = None;
        let mut result = true;

        while !min_heap.is_empty() {
            let next = min_heap.pop().unwrap().0;
            if attending.is_none() {
                attending = Some(next);
            } else {
                let current = attending.unwrap();

                if next.start >= current.end {
                    attending = Some(next);
                } else {
                    result = false;
                    break;
                }
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
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        let result = Solution::can_attend_meetings(intervals);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        let result = Solution::can_attend_meetings(intervals);
        assert!(result);
    }
}
