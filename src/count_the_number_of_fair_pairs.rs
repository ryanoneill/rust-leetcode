/// Given a 0-indexed integer array `nums` of size `n` and two integers `lower` and `upper`, return
/// the number of fair pairs.
///
/// A pair `(i, j)` is fair if:
///
/// * `0 <= i < j < n`, and
///
/// * `lower <= nums[i] + nums[j] <= upper`
struct Solution;

impl Solution {

    fn worker(nums: &Vec<i32>, value: i32) -> i64 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = 0;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum < value {
                result += (right - left) as i64;
                left += 1;
            } else {
                right -= 1;
            }
        }

        result
    }

    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort();

        Self::worker(&nums, upper+1) - Self::worker(&nums, lower)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![0,1,7,4,4,5];
        let lower = 3;
        let upper = 6;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,7,9,2,5];
        let lower = 11;
        let upper = 11;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, 1);
    }

}
