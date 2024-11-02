use std::collections::HashMap;

/// Given an integer array `nums`, return the number of non-empty subarrays with the leftmost
/// element of the subarray not larger than other elements in the subarray.
///
/// A subarray is a contiguous part of an array.
struct Solution;

impl Solution {

    pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack = Vec::new();

        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            loop {
                if stack.is_empty() {
                    break;
                } else {
                    let s = stack.len();
                    let j = stack[s-1];
                    let peek = nums[j];

                    if num < peek {
                        result += i - j;
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }
            stack.push(i);
        }

        while !stack.is_empty() {
            let index = stack.pop().unwrap();
            result += n - index;
        }

        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,4,2,5,3];
        let result = Solution::valid_subarrays(nums);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,2,1];
        let result = Solution::valid_subarrays(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let nums = vec![2,2,2];
        let result = Solution::valid_subarrays(nums);
        assert_eq!(result, 6);
    }

}
