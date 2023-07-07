/// Given an array of positive integers `nums` and a positive integer `target`, return the minimal
/// length of a subarray whose sum is greater than or equal to `target`. If there is no such
/// subarray, return `0` instead.
struct Solution;

impl Solution {

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut result = i32::MAX;

        let mut left = 0;
        for right in 0..n {
            sum += nums[right];
            while sum >= target {
                let current = right - left + 1;
                result = result.min(current as i32);
                sum -= nums[left];
                left += 1;
            }
        }

        if result == i32::MAX {
            result = 0;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let target = 7;
        let nums = vec![2,3,1,2,4,3];
        let result = Solution::min_sub_array_len(target, nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let target = 4;
        let nums = vec![1,4,4];
        let result = Solution::min_sub_array_len(target, nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let target = 11;
        let nums = vec![1,1,1,1,1,1,1,1];
        let result = Solution::min_sub_array_len(target, nums);
        assert_eq!(result, 0);
    }

}
