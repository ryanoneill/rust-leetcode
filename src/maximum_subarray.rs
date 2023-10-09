/// Given an integer array `nums`, find the subarray with the largest sum, and
/// return its sum.
struct Solution;

impl Solution {

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut result = i32::MIN;
        let mut sum = 0;

        for i in 0..n {
            sum = sum.max(0);
            let num = nums[i];
            sum += num;
            result = result.max(sum);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        let result = Solution::max_sub_array(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let result = Solution::max_sub_array(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let nums = vec![5,4,-1,7,8];
        let result = Solution::max_sub_array(nums);
        assert_eq!(result, 23);
    }

    #[test]
    fn all_negative() {
        let nums = vec![-7,-3,-5,-2,-8];
        let result = Solution::max_sub_array(nums);
        assert_eq!(result, -2);
    }

}
