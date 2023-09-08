use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    index: usize,
    day: usize,
}

impl State {

    pub fn new(index: usize, day: usize) -> Self {
        Self { index, day }
    }

}

/// You want to schedule a list of jobs in `d` days. Jobs are dependent (i.e. To work on the `ith`
/// job, you have to finish all the jobs `j` where `0 <= j < i`).
///
/// You have to finish at least one task every day. The difficulty of a job schedule is the sum of
/// difficulties of each day of the `d` days. The difficulty is the maximum difficulty of a job
/// done on that day.
///
/// You are given an integer array `jobDifficulty` and an integer `d`. The difficulty of the `ith`
/// job is `jobDifficulty[i]`.
///
/// Return the minimum difficulty of a job schedule. If you cannot find a schedule for the jobs
/// return `-1`.
struct Solution;

impl Solution {

    fn dp(results: &mut HashMap<State, i32>, job_difficulty: &Vec<i32>, d: usize, hardest_job_remaining: &Vec<i32>, state: State) -> i32 {
        if results.contains_key(&state) {
            results[&state]
        } else {
            let result;

            if state.day == d {
                result = hardest_job_remaining[state.index];
            } else {
                let n = job_difficulty.len();

                let mut best = i32::MAX;
                let mut hardest = 0;

                for j in state.index..(n - (d - state.day)) {
                    hardest = hardest.max(job_difficulty[j]);
                    let new_state = State::new(j+1, state.day + 1);
                    best = best.min(hardest + Self::dp(results, job_difficulty, d, hardest_job_remaining, new_state));
                }
                result = best
            }

            results.insert(state, result);
            result
        }
    }

    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let mut result = -1;
        let d = d as usize;
        let n = job_difficulty.len();

        if n >= d {
            let mut hardest_job_remaining = vec![0; n];
            let mut hardest_job = 0;
            for i in (0..n).rev() {
                hardest_job = hardest_job.max(job_difficulty[i]);
                hardest_job_remaining[i] = hardest_job;
            }

            let state = State::new(0, 1);
            let mut results = HashMap::new();
            result = Self::dp(&mut results, &job_difficulty, d, &hardest_job_remaining, state);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let job_difficulty = vec![6,5,4,3,2,1];
        let d = 2;
        let result = Solution::min_difficulty(job_difficulty, d);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let job_difficulty = vec![9,9,9];
        let d = 4;
        let result = Solution::min_difficulty(job_difficulty, d);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let job_difficulty = vec![1,1,1];
        let d = 3;
        let result = Solution::min_difficulty(job_difficulty, d);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_4() {
        let job_difficulty = vec![11,111,22,222,33,333,44,444];
        let d = 6;
        let result = Solution::min_difficulty(job_difficulty, d);
        assert_eq!(result, 843);
    }

}
