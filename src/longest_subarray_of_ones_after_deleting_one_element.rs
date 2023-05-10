/// Given a binary array `nums`, you should delete one element from it.
///
/// Return the size of the longest non-empty subarray containing only `1`'s in
/// the resulting array. Return `0` if there is no such subarray.
struct Solution;

impl Solution {

    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut current = 0;
        let mut max = 0;
        let mut zero_index = usize::MAX;
        let mut start = usize::MAX;

        for i in 0..n {
            let value = nums[i];
            if value == 1 {
                if start == usize::MAX {
                    start = i;
                    current = 0;
                }
                current += 1;
                max = max.max(current);
            } else if zero_index == usize::MAX {
                zero_index = i;
            } else if start == usize::MAX {
                zero_index = i;
            } else if zero_index < start {
                zero_index = i;
            } else {
                let diff = zero_index - start;
                current -= diff as i32;
                start = zero_index + 1;
                zero_index = i;
            }
        }

        max.min(n as i32 - 1)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,1,0,1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,1,1,1,0,1,1,0,1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,1,1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn all_zeroes() {
        let nums = vec![0,0,0,0];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn start_zeroes() {
        let nums = vec![0,0,0,1,1,1,1,1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn end_zeroes() {
        let nums = vec![1,1,1,1,0,0,0,0,0];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 4);
    }

}
