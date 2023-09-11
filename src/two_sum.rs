use std::collections::HashMap;

/// Given an array of integers `nums` and an integer `target`, return indices
/// of the two numbers such that they add up to `target`.
///
/// You may assume that each input would have exactly one solution, and you may
/// not use the same element twice.
///
/// You can return the answer in any order.
struct Solution;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut results = vec![0, 0];
        let mut indices: HashMap<i32, Vec<usize>> = HashMap::new();
        let n = nums.len();

        for i in 0..n {
            let num = nums[i];
            let diff = target - num;

            if indices.contains_key(&diff) {
                results[0] = indices[&diff][0] as i32;
                results[1] = i as i32;
                break;
            }

            indices
                .entry(num)
                .or_insert(Vec::new())
                .push(i);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result1 = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result1, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let result2 = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result2, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let result3 = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result3, vec![0, 1]);
    }
}
