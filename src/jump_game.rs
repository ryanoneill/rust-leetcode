/// You are given an integer array `nums`. You are initially positioned at the
/// array's first index, and each element in the array represents your maximum
/// jump length at that position.
///
/// Return `true` if you can reach the last index, or `false` otherwise.
struct Solution;

impl Solution {

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut result = false;
        let mut max_so_far = 0;
        let n = nums.len();

        for i in 0..n {
            if i > max_so_far {
                result = false;
                break;
            }
            let max_from_here = i + nums[i] as usize;
            max_so_far = max_so_far.max(max_from_here);
            if max_so_far >= n-1 {
                result = true;
                break;
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
        let result = Solution::can_jump(nums);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,2,1,0,4];
        let result = Solution::can_jump(nums);
        assert!(!result);
    }

}
