/// You are given an integer array `nums` sorted in non-decreasing order.
///
/// Build and return an integer array `result` with the same length as `nums` such that `result[i]`
/// is equal to the summation of absolute differences between `nums[i]` and all the other elements
/// in the array.
///
/// In other words, `result[i]` is equal to `sum(|nums[i]-nums[j])` where `0 <= j < nums.length`
/// and `j != i` (0-indexed).
struct Solution;

impl Solution {

    pub fn left_prefix_sum(nums: &Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut result = Vec::with_capacity(n);
        let mut sum = 0;

        for i in 0..n {
            result.push(sum);
            sum += nums[i] as i64;
        }

        result
    }

    pub fn right_prefix_sum(nums: &Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut sum = 0;

        for i in 0..n {
            let index = n-1-i;
            result[index] = sum;
            sum += nums[index] as i64;
        }

        result
    }

    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let left_sums = Self::left_prefix_sum(&nums);
        let right_sums = Self::right_prefix_sum(&nums);

        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let num = nums[i] as i64;
            let j = n-1-i;
            let left = left_sums[i];
            let right = right_sums[i];

            let left_part = (num * (i as i64)) - left;
            let right_part = right - (num * (j as i64));
            let value = left_part + right_part;
            result.push(value as i32); // Narrowing on purpose
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2,3,5];
        let result = Solution::get_sum_absolute_differences(nums);
        assert_eq!(result, vec![4,3,5]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,4,6,8,10];
        let result = Solution::get_sum_absolute_differences(nums);
        assert_eq!(result, vec![24,15,13,15,21]);
    }

}
