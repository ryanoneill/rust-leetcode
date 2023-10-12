/// You are given a sorted array consisting of only integers where every element appears exactly
/// twice, except for one element which appears exactly once.
///
/// Return the single element that appears only once.
///
/// Your solution must run in O(log n) time and O(1) space.
struct Solution;

impl Solution {

    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let n = nums.len();
        if n == 1 {
            result = nums[0];
        } else if nums[0] != nums[1] { // check first
            result = nums[0]
        } else if nums[n-1] != nums[n-2] {
            result = nums[n-1]
        } else {
            let mut left = 1;
            let mut right = n-2;

            while left <= right {
                let mid = left + (right - left) / 2;

                let value = nums[mid];
                let before = nums[mid-1];
                let after = nums[mid+1];

                if value != before && value != after {
                    result = value;
                    break;
                } else if value == before {
                    if mid % 2 == 0 {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                } else if value == after {
                    if mid % 2 == 0 {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
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
        let nums = vec![1,1,2,3,3,4,4,8,8];
        let result = Solution::single_non_duplicate(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,3,7,7,10,11,11];
        let result = Solution::single_non_duplicate(nums);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2,2,3,3,4,4];
        let result = Solution::single_non_duplicate(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let nums = vec![1,1,2,2,3,3,4,4,5];
        let result = Solution::single_non_duplicate(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_5() {
        let nums = vec![7];
        let result = Solution::single_non_duplicate(nums);
        assert_eq!(result, 7);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space ✓
// ** Input: nums: integer array
// ** Output: integer the single element that appears only once
// ** nums.len() >= 1 and <= 100_000.
// ** num is >= 0 and <= 100_000.
//
// 3. Write down the test cases
// ** Input: nums = [1,1,2,3,3,4,4,8,8]
// ** Output: 2
//
// ** Input: nums = [3,3,7,7,10,11,11]
// ** Output: 10
//
// ** Input: nums = [1,2,2,3,3,4,4]
// ** Output: 1
//
// ** Input: nums = [1,1,2,2,3,3,4,4,5]
// ** Output: 5
//
// ** Input: nums = [7]
// ** Output: 7
//
// 4. Describe and write down the algorithm
// ** If the array only contained numbers that appeared twice, then those numbers would be at
// positions i and i+1 where i is even and i+1 is odd. If the pair are at i and i+1 where i is odd
// and i+1 is even, then the single value needs to be to the left of here, otherwise it's to the
// right. 
// ** Use binary search and check nearby elements to determine where the non-duplicated element is.
// ** Time complexity:  O(log n)
// ** Space complexity: O(1)
//
// 5. Code the algorithm
