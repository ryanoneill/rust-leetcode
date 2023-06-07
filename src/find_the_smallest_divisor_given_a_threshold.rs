/// Given an array of integers `nums` and an integer `threshold`, we will
/// choose a positive integer `divisor`, divide all the array by it, and sum
/// the division's result. Find the smallest `divisor` such that the result
/// mentioned above is less than or equal to `threshold`.
///
/// Each result of the division is rounded to the nearest integer greater than
/// or equal to that element. (For example: `7/3 = 3` and `10/2 = 5`).
///
/// The test cases are generated so that there will be an answer.
struct Solution;

impl Solution {

    fn divide_by_and_sum(nums: &Vec<i32>, candidate: i32) -> i64 {
        let candidate = candidate as f64;
        let mut result = 0;
        let n = nums.len();

        for i in 0..n {
            let num = nums[i] as f64;
            let quotient = (num / candidate).ceil() as i64;
            result += quotient;
        }

        result
    }

    fn max(nums: &Vec<i32>) -> i32 {
        let mut result = 1;
        let n = nums.len();
        for i in 0..n {
            result = result.max(nums[i]);
        }
        result
    }

    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let threshold = threshold as i64;
        let mut start = 1;
        let mut end = Self::max(&nums);

        while start <= end {
            let mid = start + (end - start) / 2;
            let sum = Self::divide_by_and_sum(&nums, mid);
            if sum <= threshold {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        start
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,5,9];
        let threshold = 6;
        let result = Solution::smallest_divisor(nums, threshold);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let nums = vec![44,22,33,11,1];
        let threshold = 5;
        let result = Solution::smallest_divisor(nums, threshold);
        assert_eq!(result, 44);
    }

}
