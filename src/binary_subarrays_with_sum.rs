use std::collections::HashMap;

/// Given a binary array `nums` and an integer `goal`, return the number of non-empty subarrays
/// with a sum `goal`.
///
/// A subarray is a contiguous part of the array.
struct Solution;

impl Solution {

    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        let mut sum_freqs = HashMap::new();

        for num in nums {
            sum += num;
            if sum == goal {
                result += 1;
            }

            let diff = sum - goal;
            if sum_freqs.contains_key(&diff) {
                result += sum_freqs[&diff];
            }

            sum_freqs
                .entry(sum)
                .and_modify(|c| { *c += 1; })
                .or_insert(1);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,0,1,0,1];
        let goal = 2;
        let result = Solution::num_subarrays_with_sum(nums, goal);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,0,0,0,0];
        let goal = 0;
        let result = Solution::num_subarrays_with_sum(nums, goal);
        assert_eq!(result, 15);
    }

}
