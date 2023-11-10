/// You are given a 0-indexed integer array `nums` where `nums[i]` represents the score of the
/// `ith` student. You are also given an integer `k`.
///
/// Pick the scores of any `k` students from the array so that the difference between the highest
/// and lowest of the `k` scores is minimized.
///
/// Return the minimum possible difference.
struct Solution;

impl Solution {
    
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();

        let j = (k - 1) as usize;
        let mut result = i32::MAX;

        for i in 0..n-j {
            let first = nums[i];
            let second = nums[i+j];
            let diff = second - first;
            result = result.min(diff);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![90];
        let k = 1;
        let result = Solution::minimum_difference(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let nums = vec![9, 4, 1, 7];
        let k = 2;
        let result = Solution::minimum_difference(nums, k);
        assert_eq!(result, 2);
    }

}
