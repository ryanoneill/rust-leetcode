use std::collections::HashMap;

/// Given an array of *distinct* integers `nums` and a target integer `target`, return the number
/// of possible combinations that add up to `target`.
///
/// The test cases are generated so that the answer can fit in a 32-bit integer.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<i32, i32>, nums: &Vec<i32>, target: i32) -> i32 {
        let mut result = 0;

        if results.contains_key(&target) {
            result = results[&target];
        } else {
            for &num in nums {
                let current = target - num;
                if current == 0 {
                    result += 1;
                } else if current > 0 {
                    result += Self::worker(results, nums, current);
                }
            }
            results.insert(target, result);
        }

        result
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut results = HashMap::new();
        Self::worker(&mut results, &nums, target)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3];
        let target = 4;
        let result = Solution::combination_sum4(nums, target);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let nums = vec![9];
        let target = 3;
        let result = Solution::combination_sum4(nums, target);
        assert_eq!(result, 0);
    }

}
