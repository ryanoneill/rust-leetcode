/// Given an integer array `nums` of length `n`, you want to create an array `ans` of length `2n`
/// where `ans[i] == nums[i]` and `ans[i+n] == nums[i]` for `0 <= i < n` (0-indexed).
///
/// Specifically, `ans` is the concatenation of two `nums` arrays.
///
/// Return the array `ans`.
struct Solution;

impl Solution {

    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; 2 * n];
        for (i, num) in nums.into_iter().enumerate() {
            result[i] = num;
            result[i+n] = num;
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,1];
        let result = Solution::get_concatenation(nums);
        assert_eq!(result, vec![1,2,1,1,2,1]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,3,2,1];
        let result = Solution::get_concatenation(nums);
        assert_eq!(result, vec![1,3,2,1,1,3,2,1]);
    }

}
