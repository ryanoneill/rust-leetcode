use std::collections::HashSet;

/// Given two integer arrays `nums1` and `nums2`, return an
/// array of their intersection. Each element in the result must be unique and you may return the
/// result in any order.
struct Solution;

impl Solution {

    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let set2: HashSet<i32> = HashSet::from_iter(nums2.into_iter());

        set1.intersection(&set2).copied().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = Solution::intersection(nums1, nums2);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9,4,9,8,4];
        let mut result = Solution::intersection(nums1, nums2);
        result.sort_unstable();
        assert_eq!(result, vec![4,9]);
    }

}
