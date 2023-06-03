use std::collections::HashMap;

/// You are given two integer arrays `nums` and `nums2` where `nums2` is an anagram of `nums1`.
/// Both arrays may contain duplicates.
///
/// Return an index mapping array `mapping` from `nums1` to `nums2` where `mapping[i] = j` means
/// the `ith` element in `nums1` appears in `nums2` at index `j`. If there are multiple answres,
/// return any of them.
///
/// An array `a` is an anagram of an array `b` means `b` is made by randomizing the order of the
/// elements in `a`.
struct Solution;

impl Solution {

    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut places = HashMap::new();
        for (i, num) in nums2.into_iter().enumerate() {
            places.insert(num, i);
        }

        nums1
            .into_iter()
            .map(|num1| places[&num1] as i32)
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![12,28,46,32,50];
        let nums2 = vec![50,12,32,46,28];
        let result = Solution::anagram_mappings(nums1, nums2);
        assert_eq!(result, vec![1,4,3,2,0]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![84,46];
        let nums2 = vec![84,46];
        let result = Solution::anagram_mappings(nums1, nums2);
        assert_eq!(result, vec![0,1]);
    }

}
