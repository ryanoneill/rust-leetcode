use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Meeting {
    start: i32,
    end: i32,
}

impl Meeting {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ByEnd {
    meeting: Meeting,
}

impl ByEnd {
    fn new(meeting: Meeting) -> Self {
        Self { meeting }
    }
}

impl Ord for ByEnd {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.meeting.end < other.meeting.end {
            Ordering::Less
        } else if self.meeting.end > other.meeting.end {
            Ordering::Greater
        } else if self.meeting.start < other.meeting.start {
            Ordering::Less
        } else if self.meeting.start > other.meeting.start {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for ByEnd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Given an array of meeting time intervals `intervals` where
/// `intervals[i] = [starti, endi]`, return the minimum number of conference
/// rooms required.
struct Solution;

impl Solution {
    fn order_meetings_by_start(intervals: Vec<Vec<i32>>) -> BinaryHeap<Reverse<Meeting>> {
        let mut min_heap = BinaryHeap::new();
        for interval in intervals {
            let meeting = Meeting::new(interval[0], interval[1]);
            min_heap.push(Reverse(meeting));
        }
        min_heap
    }

    fn remove_finished_meetings(attending: &mut BinaryHeap<Reverse<ByEnd>>, time: i32) {
        loop {
            if !attending.is_empty() {
                let next_to_finish = attending.peek().unwrap().0.meeting;
                if next_to_finish.end <= time {
                    attending.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut meetings = Self::order_meetings_by_start(intervals);
        let mut attending = BinaryHeap::new();

        while !meetings.is_empty() {
            let next = meetings.pop().unwrap().0;
            Self::remove_finished_meetings(&mut attending, next.start);
            attending.push(Reverse(ByEnd::new(next)));
            result = result.max(attending.len() as i32);
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
        let result = Solution::min_meeting_rooms(intervals);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        let result = Solution::min_meeting_rooms(intervals);
        assert_eq!(result, 1);
    }
}
