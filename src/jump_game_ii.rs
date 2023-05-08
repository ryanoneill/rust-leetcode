/// You are given a 0-indexed array of integers `nums` of length `n`. You are
/// initially positioned at `nums[0]`.
///
/// Each element `nums[i]` represents the maximum length of a forward jump
/// from index `i`. In other words, if you are at `nums[i]`, you can jump
/// to any `nums[i+j]` where:
///
/// * `0 <= j <= nums[i]` and
///
/// * `i + j < n`
///
/// Return the minimum number of jumps to reach `nums[n-1]`. The test cases
/// are generated such that you can reach `nums[n-1]`.
struct Solution;

impl Solution {

    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        let mut end = 0;
        let mut farthest = 0;

        for i in 0..n-1 {
            let max_from_here = i + nums[i] as usize;
            farthest = farthest.max(max_from_here);

            if i == end {
                result += 1;
                end = farthest;
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
        let nums = vec![2,3,1,1,4];
        let result = Solution::jump(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,3,0,1,4];
        let result = Solution::jump(nums);
        assert_eq!(result, 2);
    }

}
