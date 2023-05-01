use std::collections::HashMap;

/// Given an array `nums` of size `n`, return the majority element.
///
/// The majority element is the element that appears more than `⌊n / 2⌋` times.
/// You may assume that the majority element always exists in the array.
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for num in nums {
            counts
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        counts
            .into_iter()
            .max_by_key(|item| item.1)
            .map(|item| item.0)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3, 2, 3];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 2);
    }
}
