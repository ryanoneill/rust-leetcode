use std::collections::BTreeSet;

/// Given two 0-indexed integer arrays `nums` and `nums2`, return a list
/// `answer` of size `2` where:
///
/// * `answer[0]` is a list of all distinct integers in `nums1` which are not
///   present in `nums2`.
///
/// * `answer[1]` is a list of all distinct integers in `nums2` which are not
///   present in `nums1`.
///
/// Note that the integers in the lists may be returned in any order.
struct Solution;

impl Solution {

    // BTreeSet is not necessary here as "the lists may be returned in any order",
    // but I wanted to have some practice using it.
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1_set = BTreeSet::from_iter(nums1.into_iter());
        let nums2_set = BTreeSet::from_iter(nums2.into_iter());

        let nums1_result: Vec<i32> = nums1_set.difference(&nums2_set).into_iter().copied().collect();
        let nums2_result: Vec<i32> = nums2_set.difference(&nums1_set).into_iter().copied().collect();

        vec![nums1_result, nums2_result]
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let result = Solution::find_difference(nums1, nums2);
        assert_eq!(result, vec![vec![1,3], vec![4,6]]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![1,2,3,3];
        let nums2 = vec![1,1,2,2];
        let result = Solution::find_difference(nums1, nums2);
        assert_eq!(result, vec![vec![3], vec![]]);
    }

}
