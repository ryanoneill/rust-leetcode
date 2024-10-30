/// You are given an integer array `nums`. The unique elements of an array are the elements that
/// appear exactly once in the array.
///
/// Return the sum of all the unique elements of `nums`.
use std::collections::HashSet;

struct Solution;

impl Solution {

    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut multiple = HashSet::new();
        let mut once = HashSet::new();

        for num in nums {
            if !multiple.contains(&num) {
                if once.contains(&num) {
                    once.remove(&num);
                    multiple.insert(num);
                } else {
                    once.insert(num);
                }
            }
        }

        for num in once {
            result += num;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,2];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,1,1,1,1];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2,3,4,5];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 15);
    }

}
