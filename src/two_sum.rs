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
        let mut values: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());
        for (i, value) in nums.iter().enumerate() {
            match values.get_mut(value) {
                Some(indices) => {
                    indices.push(i);
                }
                None => {
                    values.insert(*value, vec![i]);
                }
            }
        }
        let mut result = Vec::with_capacity(2);
        for num in nums {
            let diff = target - num;
            match (values.get(&num), values.get(&diff)) {
                (Some(is), Some(_)) if num == diff => {
                    if is.len() >= 2 {
                        result.push(is[0] as i32);
                        result.push(is[1] as i32);
                        return result;
                    }
                }
                (Some(is), Some(js)) => {
                    result.push(is[0] as i32);
                    result.push(js[0] as i32);
                    return result;
                }
                (_, _) => {
                    // do nothing, keep going
                }
            }
        }
        result
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
