use std::cmp::max;
use std::collections::HashMap;

/// Given an integer array `nums`, return the length of the longest strictly
/// increasing subsequence.
struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lengths = HashMap::new();
        let mut result = 0;

        for i in 0..nums.len() {
            result = max(result, Self::worker(i, &nums, &mut lengths));
        }

        result
    }

    fn worker(i: usize, nums: &[i32], lengths: &mut HashMap<usize, i32>) -> i32 {
        if lengths.contains_key(&i) {
            lengths[&i]
        } else {
            let mut result = 1;
            for j in 0..i {
                if nums[i] > nums[j] {
                    result = max(result, Self::worker(j, nums, lengths) + 1);
                }
            }
            lengths.insert(i, result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 1);
    }
}
