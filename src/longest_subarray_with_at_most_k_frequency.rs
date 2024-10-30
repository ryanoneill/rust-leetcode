use std::collections::HashMap;

/// You are given an integer array `nums` and an integer `k`.
///
/// The frequency of an element `x` is the number of times it occurs in an array.
///
/// An array is called good if the frequency of each element in this array is less than or equal to
/// `k`.
///
/// Return the length of the longest good subarray of `nums`.
///
/// A subarray is a contiguous non-empty sequence of elements within an array.
struct Solution;

impl Solution {

    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut result = 0;
        let mut counts = HashMap::new();

        let mut current = 0;

        let mut left = 0;

        for right in 0..n {
            current += 1;
            let num = nums[right];
            counts
                .entry(num)
                .and_modify(|c| { *c += 1; })
                .or_insert(1);
            if counts[&num] <= k {
                result = result.max(current);
            } else {
                while counts[&num] > k {
                    let num = nums[left];
                    counts
                        .entry(num)
                        .and_modify(|c| { *c -= 1; });
                    current -= 1;
                    left += 1;
                }
            }
        } 

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,1,2,3,1,2];
        let k = 2;
        let result = Solution::max_subarray_length(nums, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,1,2,1,2,1,2];
        let k = 1;
        let result = Solution::max_subarray_length(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let nums = vec![5,5,5,5,5,5,5];
        let k = 4;
        let result = Solution::max_subarray_length(nums, k);
        assert_eq!(result, 4);
    }

}
