/// Given an integer array `nums` containing distinct positive integers, find and return any number
/// from the array that is neither the minimum nor the maximum value in the array, or `-1` if there
/// is no such number.
///
/// Return the selected integer.
struct Solution;

impl Solution {

    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 { -1 }
        else {
            let mut min = nums[0];
            let mut max = nums[0];

            if nums[1] > max {
                max = nums[1];
            } else {
                min = nums[1];
            }

            if nums[2] > max {
                max
            } else if nums[2] < min {
                min
            } else {
                nums[2]
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,2,1,4];
        let result = Solution::find_non_min_or_max(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2];
        let result = Solution::find_non_min_or_max(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let nums = vec![2,1,3];
        let result = Solution::find_non_min_or_max(nums);
        assert_eq!(result, 2);
    }

}
