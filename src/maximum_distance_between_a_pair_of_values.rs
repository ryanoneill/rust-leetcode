/// You are given two non-increasing 0-indexed integer arrays `nums1` and `nums2`.
///
/// A pair of indices `(i, j)`, where `0 <= i < nums1.length` and `0 <= j < nums2.length`, is valid
/// if both `i <= j` and `nums1[i] <= nums2[j]`. The distance of the pair is `j - i`.
///
/// Return the maximum distance of any valid pair `(i, j)`. If there are no valid pairs, return
/// `0`.
///
/// An array `arr` is non-increasing if `arr[i-1] >= arr[i]` for every `1 <= i < arr.length`.
struct Solution;

impl Solution {

    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;

        let m = nums1.len();
        let n = nums2.len();

        let mut i = 0;
        let mut j = 0;

        while i < m && j < n {
            let num1 = nums1[i];
            let num2 = nums2[j];

            if num1 > num2 {
                i += 1;
            } else {
                let distance = j as i32 - i as i32;
                result = result.max(distance);
                j += 1;
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
        let nums1 = vec![55,30,5,4,2];
        let nums2 = vec![100,20,10,10,5];
        let result = Solution::max_distance(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![2,2,2];
        let nums2 = vec![10,10,1];
        let result = Solution::max_distance(nums1, nums2);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let nums1 = vec![30,29,19,5];
        let nums2 = vec![25,25,25,25,25];
        let result = Solution::max_distance(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_4() {
        let nums1 = vec![9819,9508,7398,7347,6337,5756,5493,5446,5123,3215,1597,774,368,313];
        let nums2 = vec![9933,9813,9770,9697,9514,9490,9441,9439,8939,8754,8665,8560];
        let result = Solution::max_distance(nums1, nums2);
        assert_eq!(result, 9);
    }

}
