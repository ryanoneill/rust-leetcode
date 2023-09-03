/// Given an integer array `nums` sorted in non-decreasing order and an integer `target`, return
/// `true` if `target` is a majority element, or `false` otherwise.
///
/// A majority element in an array `nums` is an element that appears more than `nums.length / 2`
/// times in the array.
struct Solution;

impl Solution {

    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let mut first = n+1;

        let mut left = 0;
        let mut right = n - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let value = nums[mid];
            if value == target {
                if mid == 0 || nums[mid-1] != target {
                    first = mid;
                    break;
                } else {
                    right = mid - 1;
                }
            } else if value < target {
                if mid == n-1 {
                    break;
                } else if nums[mid+1] == target {
                    first = mid + 1;
                    break;
                } else {
                    left = mid + 2;
                }
            } else if mid == 0 {
                break;
            } else {
                right = mid - 1;
            }
        }

        let half = n / 2;
        let last = first + half;

        last < n && nums[last] == target
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2,4,5,5,5,5,5,6,6];
        let target = 5;
        let result = Solution::is_majority_element(nums, target);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![10,100,101,101];
        let target = 101;
        let result = Solution::is_majority_element(nums, target);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let nums = vec![438885258];
        let target = 786460391;
        let result = Solution::is_majority_element(nums, target);
        assert!(!result);
    }
    
}
