use std::collections::HashSet;

/// Given an integer array `nums`, return `true` if any value appears
/// at least twice in the array, and retun `false` if every element
/// is distinct.
struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        let mut result = false;

        for num in nums {
            if !seen.insert(num) {
                result = true;
                break;
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
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(nums);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::contains_duplicate(nums);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = Solution::contains_duplicate(nums);
        assert!(result);
    }
}
