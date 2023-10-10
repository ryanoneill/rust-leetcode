use std::collections::HashSet;

/// Given an array `nums` of `n` integers where `nums[i]` is in the range `[1,n]`, return an array
/// of all the integers in the range `[1,n]` that do not appear in `nums`.
struct Solution;

impl Solution {

    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let seen: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let m = n - seen.len();

        let mut result = Vec::with_capacity(m);
        for i in 1..=n {
            let num = i as i32;
            if !seen.contains(&num) {
                result.push(num);
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
        let nums = vec![4,3,2,7,8,2,3,1];
        let result = Solution::find_disappeared_numbers(nums);
        assert_eq!(result, vec![5,6]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,1];
        let result = Solution::find_disappeared_numbers(nums);
        assert_eq!(result, vec![2]);
    }

}
