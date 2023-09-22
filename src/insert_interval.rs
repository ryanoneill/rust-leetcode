#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    Before,
    Within,
    After,
}

/// You are given an array of non-overlapping intervals `intervals` where `intervals[i] = [starti,
/// endi]` represent the start and end of the `ith` interval and `intervals` is sorted in ascending
/// order by `starti`. You are also given an interval `newInterval = [start, end]` that represents
/// the start and end of another interval.
///
/// Insert `newInterval` into `intervals` such that `intervals` is still stored in ascending order
/// by `starti` and `intervals` still does not have any overlapping intervals (merge overlapping
/// intervals if necessary).
///
/// Return `intervals` after the insertion.
struct Solution;

impl Solution {

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut state = State::Before;
        let mut results = Vec::new();

        let n = intervals.len();

        let new_begin = new_interval[0];
        let new_end = new_interval[1];

        let mut combined_begin = new_begin;
        let mut combined_end = new_end;

        println!("{:?}", new_interval);

        for i in 0..n {
            let current = &intervals[i];
            let current_begin = current[0];
            let current_end = current[1];

            println!("{:?} {:?}", current, state);
            if state == State::Before {
                if new_begin < current_begin {
                    if new_end < current_begin {
                        state = State::After;
                        results.push(vec![new_begin, new_end]);
                        results.push(current.clone());
                    } else if new_end <= current_end {
                        state = State::After;
                        results.push(vec![new_begin, current_end]);
                    } else {
                        state = State::Within;
                        combined_begin = new_begin;
                        combined_end = new_end.max(current_end);
                    }
                } else if new_begin <= current_end {
                    if new_end <= current_end {
                        state = State::After;
                        results.push(vec![current_begin, current_end]);
                    } else {
                        state = State::Within;
                        combined_begin = current_begin;
                        combined_end = new_end;
                    }
                } else {
                    results.push(current.clone());
                }
            } else if state == State::Within {
                if combined_end < current_begin {
                    state = State::After;
                    results.push(vec![combined_begin, combined_end]);
                    results.push(current.clone());
                } else if combined_end <= current_end {
                    state = State::After;
                    results.push(vec![combined_begin, current_end]);
                } else {
                    // Keep Going
                }
            } else {
                results.push(current.clone());
            }
        }

        if state != State::After {
            results.push(vec![combined_begin, combined_end]);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1,3], vec![6,9]];
        let new_interval = vec![2,5];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1,5], vec![6,9]]);
    }

    #[test]
    fn example_2() {
        let intervals = vec![vec![1,2], vec![3,5], vec![6,7], vec![8,10], vec![12,16]];
        let new_interval = vec![4,8];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1,2], vec![3,10], vec![12,16]]);
    }

    #[test]
    fn example_3() {
        let intervals = vec![];
        let new_interval = vec![5,7];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![5,7]]);
    }

}
