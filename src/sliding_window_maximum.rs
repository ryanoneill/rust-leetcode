use std::collections::VecDeque;

/// You are given an array of integers `nums`, there is a sliding window of
/// size `k` which is moving from the very left of the array to the very right.
/// You can only see the `k` numbers in the window. Each time the sliding window
/// moves right by one position.
///
/// Return the max sliding window.
struct Solution;

impl Solution {

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        let mut maxes: VecDeque<usize> = VecDeque::new();

        for (i, num) in nums.iter().enumerate() {
            while !maxes.is_empty() && *num > nums[*maxes.back().unwrap()] {
                maxes.pop_back();
            }
            maxes.push_back(i);

            if maxes.front().unwrap() + k == i {
                maxes.pop_front();
            }

            if i >= k - 1 {
                result.push(nums[*maxes.front().unwrap()]);
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
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, vec![1]);
    }

}
