use std::collections::HashMap;

/// Given a binary array `nums`, return the maximum length of a contiguous subarray with an equay
/// number of `0` and `1`.
struct Solution;

impl Solution {

    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut counts = HashMap::new();
        let mut count = 0;

        let mut result = 0;

        for i in 0..n {
            let num = nums[i];
            if num == 1 {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                result = result.max(i as i32 + 1);
            } else if counts.contains_key(&count) {
                let diff = i - counts[&count];
                result = result.max(diff as i32);
            } else {
                counts.insert(count, i);
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
        let nums = vec![0, 1];
        let result = Solution::find_max_length(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 1, 0];
        let result = Solution::find_max_length(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let nums = vec![0,0,0,0,0,1,1,1,1,1,0,1,1,1];
        let result = Solution::find_max_length(nums);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_4() {
        let nums = vec![1,1,1,1,1];
        let result = Solution::find_max_length(nums);
        assert_eq!(result, 0);
    }

}
