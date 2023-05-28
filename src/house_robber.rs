/// You are a professional robber planning to rob houses along a street. Each
/// house has a certain amount of money stashed, the only constraint stopping
/// you from robbing each of them is that adjacent houses have security systems
/// connected and it will automatically contact police if two adjacent houses
/// were broken into on the same night.
///
/// Given an integer array `nums` representing the amount of money of each
/// house, return the maximum amount of money you can rob tonight without
/// alerting the police.
struct Solution;

impl Solution {

    // Switched to bottom-up iterative solution
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            nums[0]
        } else {
            let mut most = vec![0; n];
            most[0] = nums[0];
            most[1] = nums[0].max(nums[1]);

            for i in 2..n {
                let sum_with = nums[i] + most[i-2];
                let sum_without = most[i-1];
                most[i] = sum_with.max(sum_without);
            }
            most[n-1]
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::rob(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let result = Solution::rob(nums);
        assert_eq!(result, 12);
    }
}
