/// Given a sorted array of distinct integers and a target value, return the
/// index if the target is found. If not, return the index where it would be if
/// it were inserted in order.
///
/// You must write an algorithm with `O(log n)` runtime complexity.
struct Solution;

impl Solution {
    fn worker(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> usize {
        if start >= end {
            start
        } else {
            let mid = start + (end - start) / 2;
            let value = nums[mid];
            if value == target {
                mid
            } else if value < target {
                if mid == nums.len() - 1 {
                    mid + 1
                } else if nums[mid + 1] > target {
                    mid + 1
                } else {
                    Self::worker(&nums, target, mid + 1, end)
                }
            } else {
                if mid == 0 {
                    0
                } else if nums[mid - 1] < target {
                    mid
                } else {
                    Self::worker(&nums, target, start, mid - 1)
                }
            }
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            0
        } else if target < nums[0] {
            0
        } else if target > nums[nums.len() - 1] {
            nums.len() as i32
        } else {
            Self::worker(&nums, target, 0, nums.len() - 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn beginning() {
        let nums = vec![5, 6, 7, 8, 9, 10];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn middle() {
        let nums = vec![1, 3];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);
    }
}
