use std::cmp::max;
use std::collections::HashMap;

/// You are a professional robber planning to rob houses along a street. Each
/// house has a certain amount of money stashed, the only constraint stopping
/// you from robbing each of them is that adjacent houses have security systems
/// connected and it will automatically contact police if two adjacent houses
/// were broken into on the same night.
///
/// Given an integer array `nums` representing the amount of money of each
/// house, return the maximum amount of money you can rob tonight without
/// alerting the police.
struct Solution;

impl Solution {

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut by_length = HashMap::new();
        Self::worker(&nums, &mut by_length)
    }

    fn worker(nums: &[i32], by_length: &mut HashMap<usize, i32>) -> i32 {
        let n = nums.len();
        match n {
            0 => 0,
            1 => nums[0],
            _ => {
                if by_length.contains_key(&n) {
                    by_length[&n]
                } else {
                    let sum_with = nums[0] + Self::worker(&nums[2..], by_length);
                    let sum_without = Self::worker(&nums[1..], by_length);
                    let result = max(sum_with, sum_without);
                    by_length.insert(n, result);

                    result
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,1];
        let result = Solution::rob(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,7,9,3,1];
        let result = Solution::rob(nums);
        assert_eq!(result, 12);
    }

}
