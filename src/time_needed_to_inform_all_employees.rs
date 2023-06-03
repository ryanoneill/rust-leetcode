use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    employee: i32,
    time: i32
}

impl State {

    fn new(employee: i32, time: i32) -> Self {
        Self { employee, time }
    }

}

/// A company has `n` employees with a unique ID for each employee from `0` to `n - 1`. The head of
/// the company is the one with `headID`.
///
/// Each employee has one direct manager given in the `manager` array where `manager[i]` is the
/// direct manager of the `i-th` employee, `manager[headID] = -1`. Also, it is guaranteed that the
/// subordination relationships have a tree structure.
///
/// The head of the company wants to inform all the company employees of an urgent piece of news.
/// He will inform his direct subordinates, and they will inform their subordinates, and so on
/// until all employees know about the urgent news.
///
/// The `i-th` employee needs `informTime[i]` minutes to inform all of his direct subordinates
/// (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news).
///
/// Return the number of minutes needed to inform all the employees about the urgent news.
struct Solution;

impl Solution {

    fn find_directs(manager: Vec<i32>) -> HashMap<i32, Vec<i32>> {
        let mut result = HashMap::new();

        for (i, boss) in manager.into_iter().enumerate() {
            let employee = i as i32;
            result
                .entry(boss)
                .or_insert(Vec::new())
                .push(employee);
        }

        result
    }

    pub fn num_of_minutes(_n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut result = 0;

        let directs = Self::find_directs(manager);
        let mut queue = VecDeque::new();
        let state = State::new(head_id, inform_time[head_id as usize]);
        queue.push_back(state);
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let state = queue.pop_front().unwrap();
                result = result.max(state.time);
                if directs.contains_key(&state.employee) {
                    for direct in &directs[&state.employee] {
                        let employee = *direct;
                        let time = inform_time[employee as usize];
                        let state = State::new(employee, state.time + time);
                        queue.push_back(state);
                    }
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
        let n = 1;
        let head_id = 0;
        let manager = vec![-1];
        let inform_time = vec![0];
        let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let head_id = 2;
        let manager = vec![2,2,-1,2,2,2];
        let inform_time = vec![0,0,1,0,0,0];
        let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
        assert_eq!(result, 1);
    }

    #[test]
    fn fuller_example() {
        let n = 6;
        let head_id = 2;
        let manager = vec![1,2,-1,0,1,4];
        let inform_time = vec![7,3,2,0,1,0];
        let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
        assert_eq!(result, 12);
    }

    #[test]
    fn real_world_example() {
        let n = 11;
        let head_id = 4;
        let manager = vec![5,9,6,10,-1,8,9,1,9,3,4];
        let inform_time = vec![0,213,0,253,686,170,975,0,261,309,337];
        let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
        assert_eq!(result, 2560);
    }

}
