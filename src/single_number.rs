use std::collections::HashSet;

/// Given a non-empty array of integers `nums`, every element appears twice
/// except for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use
/// only constant extra space.
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^= num;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let nums = vec![1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }
}
