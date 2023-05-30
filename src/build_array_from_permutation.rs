/// Given a zero-based permutation `nums` (0-indexed), buidl an array `ans` of
/// the same length where `ans[i] = nums[nums[i]]` for each
/// `0 <= i < nums.length` and return it.
///
/// A zero-based permutation `nums` is an array of distinct integers from `0`
/// to `nums.length - 1` (inclusive).
struct Solution;

impl Solution {

    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        for (i, num) in nums.iter().enumerate() {
            result[i] = nums[*num as usize];
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![0,2,1,5,3,4];
        let result = Solution::build_array(nums);
        assert_eq!(result, vec![0,1,2,4,5,3]);
    }

    #[test]
    fn example_2() {
        let nums = vec![5,0,1,2,3,4];
        let result = Solution::build_array(nums);
        assert_eq!(result, vec![4,5,0,1,2,3]);
    }

}
