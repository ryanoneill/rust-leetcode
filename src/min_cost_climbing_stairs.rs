use std::cmp::min;

/// You are given an integer array `cost` where `cost[i]` is the cost of the
/// `ith` step on a staircase. Once you pay the cost, you can either climb one
/// or two steps.
///
/// You can either start from the step with index `0`, or the step with index `1`.
///
/// Return the minimum cost to reach the top of the floor.
struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut mins = vec![0; n + 1];

        for i in 2..=n {
            mins[i] = min(mins[i - 1] + cost[i - 1], mins[i - 2] + cost[i - 2]);
        }

        mins[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let cost = vec![10, 15, 20];
        let result = 15;
        assert_eq!(result, Solution::min_cost_climbing_stairs(cost));
    }

    #[test]
    fn example_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let result = 6;
        assert_eq!(result, Solution::min_cost_climbing_stairs(cost));
    }
}
