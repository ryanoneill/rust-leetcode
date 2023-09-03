use std::collections::VecDeque;

/// Given an array of integers `nums` and an integer `limit`, return the size of the longest
/// non-empty subarray such that the absolute difference between any two elements of this subarray
/// is less than or equal to `limit`.
struct Solution;

impl Solution {

    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut mins = VecDeque::new();
        let mut maxes = VecDeque::new();

        for j in 0..n {
            let num = nums[j];
            while !mins.is_empty() {
                let last = *mins.back().unwrap();
                if num < last {
                    mins.pop_back();
                } else {
                    break;
                }
            }

            while !maxes.is_empty() {
                let last = *maxes.back().unwrap();
                if num > last {
                    maxes.pop_back();
                } else {
                    break;
                }
            }

            mins.push_back(num);
            maxes.push_back(num);

            if maxes[0] - mins[0] > limit {
                if mins[0] == nums[i] {
                    mins.pop_front();
                }
                if maxes[0] == nums[i] {
                    maxes.pop_front();
                }
                i += 1;
            }
        }

        (n - i) as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![8,2,4,7];
        let limit = 4;
        let result = Solution::longest_subarray(nums, limit);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![10,1,2,4,7,2];
        let limit = 5;
        let result = Solution::longest_subarray(nums, limit);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let nums = vec![4,2,2,2,4,4,2,2];
        let limit = 0;
        let result = Solution::longest_subarray(nums, limit);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_4() {
        let nums = vec![1,5,6,7,8,10,6,5,6];
        let limit = 4;
        let result = Solution::longest_subarray(nums, limit);
        assert_eq!(result, 5);
    }

}
