use std::collections::HashMap;

/// Given an array of integers `nums` and an integer `k`, return the total number of subarrays
/// whose sum equals to `k`.
///
/// A subarray is a contiguous non-empty sequence of elements within an array.
struct Solution;

impl Solution {

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();
        counts.insert(0, 1);

        let mut result = 0;
        let mut current = 0;

        for num in nums {
            current += num;
            let key = current - k;
            if counts.contains_key(&key) {
                result += counts[&key];
            }
            counts
                .entry(current)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,1,1];
        let k = 2;
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3];
        let k = 3;
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, 2);
    }

}
