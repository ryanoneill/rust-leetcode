use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Pair {
    value: i64,
    multiplier: i64,
}

impl Pair {

    pub fn new(value: i64, multiplier: i64) -> Self {
        Self { value, multiplier }
    }

}

/// You are given two 0-indexed integer arrays `nums1` and `nums2` of equal
/// length `n` and a positive integer `k`. You must choose a subsequence of
/// indices from `nums1` of length `k`.
///
/// For chosen indices `i0`, `i1`, ..., `ik-1`, your score is defined as:
///
/// * The sum of the selected elements from `nums1` multiplied with the minimum
///   of the selected elements from `nums2`.
///
/// * If can defined simply as
///   `(nums1[i0] + nums1[i1] + ... + nums1[ik-1]) * min(nums2[i0], nums2[i1], ... , nums2[ik-1]).
///
/// Return the maximum possible score.
///
/// A subsequence of indices of an array is a set that can be derived from the
/// set `{0, 1, ..., n-1}` by deleting some or no elements.
struct Solution;

impl Solution {

    fn build_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Pair> {
        let mut pairs = Vec::new();
        let n = nums1.len();
        for i in 0..n {
            let value = nums1[i] as i64;
            let multiplier = nums2[i] as i64;
            let pair = Pair::new(value, multiplier);
            pairs.push(pair);
        }
        pairs
    }

    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let n = nums1.len();
        let k = k as usize;
        let mut pairs = Self::build_pairs(nums1, nums2);
        pairs.sort_by_key(|pair| Reverse(pair.multiplier));

        let mut heap = BinaryHeap::new();
        let mut sum: i64 = 0;
        for pair in pairs.iter().take(k) {
            sum += pair.value;
            heap.push(Reverse(pair.value));
        }

        let mut result = sum * pairs[k-1].multiplier;
        for pair in pairs.iter().take(n).skip(k) {
            let smallest = heap.pop().unwrap().0;
            sum -= smallest;

            sum += pair.value;
            heap.push(Reverse(pair.value));

            let candidate = sum * pair.multiplier;
            result = result.max(candidate);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![1,3,3,2];
        let nums2 = vec![2,1,3,4];
        let k = 3;
        let result = Solution::max_score(nums1, nums2, k);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![4,2,3,1,1];
        let nums2 = vec![7,5,10,9,6];
        let k = 1;
        let result = Solution::max_score(nums1, nums2, k);
        assert_eq!(result, 30);
    }

    #[test]
    fn real_world() {
        let nums1 = vec![2,1,14,12];
        let nums2 = vec![11,7,13,6];
        let k = 3;
        let result = Solution::max_score(nums1, nums2, k);
        assert_eq!(result, 168);
    }

}
