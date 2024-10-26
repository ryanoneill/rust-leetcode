/// You are a professional robber planning to rob houses along a street. Each house has a certain
/// amount of money stashed. All houses at this place are *arranged in a circle*. That means the
/// first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system
/// connected, and *it will automatically contact the police if two adjacent houses were broken into
/// on the same night*.
///
/// Given an integer array `nums` representing the amount of money of each house, return the
/// maximum amount of money you can rob tonight *without alerting the police*.
struct Solution;

impl Solution {

    fn circle_rob(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let n = end - start;

        if n == 1 {
            nums[start]
        } else {
            let mut most = vec![0; n+1];
            most[start] = nums[start];
            most[start+1] = nums[start].max(nums[start+1]);

            for i in start+2..end {
                let sum_with = nums[i] + most[i-2];
                let sum_without = most[i-1];
                most[i] = sum_with.max(sum_without);
            }
            most[end-1]
        }
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        let result;
        if n == 1 {
            result = Self::circle_rob(&nums, 0, n);
        } else {
            let first = Self::circle_rob(&nums, 0, n-1);
            let second = Self::circle_rob(&nums, 1, n);

            result = first.max(second);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2, 3, 2];
        let result = Solution::rob(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::rob(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 2, 3];
        let result = Solution::rob(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_4() {
        let nums = vec![10];
        let result = Solution::rob(nums);
        assert_eq!(result, 10);
    }

}
