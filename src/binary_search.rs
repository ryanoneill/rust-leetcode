/// Given an array of integers `nums` which is sorted in ascending order, and
/// an integer `target`, write a function to search `target` in `nums`. If
/// `target` exists, then return its index. Otherwise, return `-1`.
///
/// You must write an algorithm with `O(log n)` runtime complexity.
struct Solution;

impl Solution {
    fn search_range(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        let mid = start + (end - start) / 2;
        let value = nums[mid];
        if value == target {
            mid as i32
        } else if value > target {
            if mid == start {
                -1
            } else {
                Self::search_range(nums, target, start, mid - 1)
            }
        } else {
            if mid == end {
                -1
            } else {
                Self::search_range(nums, target, mid + 1, end)
            }
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() > 0 {
            Self::search_range(&nums, target, 0, nums.len() - 1)
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn beginning() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = -1;
        let result = Solution::search(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn end() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 12;
        let result = Solution::search(nums, target);
        assert_eq!(result, 5);
    }

    #[test]
    fn only_yes() {
        let nums = vec![1];
        let target = 1;
        let result = Solution::search(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn only_no() {
        let nums = vec![1];
        let target = -1;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }
}
