/// Given an array `nums`. We define a running sum of an array as
/// `runningSum[i] = sum(nums[0]..nums[i])`.
///
/// Return the running sum of `nums`.
struct Solution;

impl Solution {

    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum: i32 = 0;
        nums.iter().map(|x| { sum += x; sum }).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,4];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![1,3,6,10]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,1,1,1,1];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![1,2,3,4,5]);
    }

    #[test]
    fn example_3() {
        let nums = vec![3,1,2,10,1];
        let result = Solution::running_sum(nums);
        assert_eq!(result, vec![3,4,6,16,17]);
    }

}
