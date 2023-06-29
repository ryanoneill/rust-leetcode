use std::collections::HashSet;

/// Given an integer array `nums`, return the largest integer that only occurs once. If no integer
/// occurs once, return `-1`.
struct Solution;

impl Solution {

    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut unique = HashSet::new();
        let mut dups = HashSet::new();

        for num in nums {
            if !dups.contains(&num) {
                if unique.contains(&num) {
                    dups.insert(num);
                    unique.remove(&num);
                } else {
                    unique.insert(num);
                }
            }
        }

        unique.into_iter().max().unwrap_or(-1)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![5,7,3,9,4,9,8,3,1];
        let result = Solution::largest_unique_number(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let nums = vec![9,9,8,8];
        let result = Solution::largest_unique_number(nums);
        assert_eq!(result, -1);
    }

}
