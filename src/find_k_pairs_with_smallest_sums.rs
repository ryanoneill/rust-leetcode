use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    sum: i32,
    index1: usize,
    index2: usize,
}

impl State {

    pub fn new(sum: i32, index1: usize, index2: usize) -> Self {
        Self {
            sum,
            index1,
            index2,
        }
    }

}

/// You are given two integer arrays `nums1` and `nums2` sorted in non-decreasing order and an
/// integer `k`.
///
/// Define a pair `(u, v)` which consists of one element from the first array and one element from
/// the second array.
///
/// Return the `k` pairs `(u1, v1), (u2, v2) ..., (uk, vk)` with the smallest sums.
struct Solution;

impl Solution {

    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n1 = nums1.len();
        let n2 = nums2.len();

        let mut k = k as usize;
        let mut results = Vec::with_capacity(k);
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();

        let initial = State::new(nums1[0] + nums2[0], 0, 0);
        seen.insert(initial);
        heap.push(Reverse(initial));

        while k > 0 && !heap.is_empty() {
            let state = heap.pop().unwrap().0;
            let item = vec![nums1[state.index1], nums2[state.index2]];
            results.push(item);
            k -= 1;

            if state.index1 < n1-1 {
                let i1 = state.index1 + 1;
                let i2 = state.index2;
                let sum = nums1[i1] + nums2[i2];
                let next1 = State::new(sum, i1, i2);
                if !seen.contains(&next1) {
                    seen.insert(next1);
                    heap.push(Reverse(next1));
                }
            }
            if state.index2 < n2-1 {
                let i1 = state.index1;
                let i2 = state.index2 + 1;
                let sum = nums1[i1] + nums2[i2];
                let next2 = State::new(sum, i1, i2);
                if !seen.contains(&next2) {
                    seen.insert(next2);
                    heap.push(Reverse(next2));
                }
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![1,7,11];
        let nums2 = vec![2,4,6];
        let k = 3;
        let results = Solution::k_smallest_pairs(nums1, nums2, k);
        assert_eq!(results, vec![
            vec![1,2],
            vec![1,4],
            vec![1,6],
        ]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![1,1,2];
        let nums2 = vec![1,2,3];
        let k = 3;
        let results = Solution::k_smallest_pairs(nums1, nums2, k);
        assert_eq!(results, vec![
            vec![1,1],
            vec![1,1],
            vec![1,2],
        ]);
    }

    #[test]
    fn example_3() {
        let nums1 = vec![1,2,4,5,6];
        let nums2 = vec![3,5,7,9];
        let k = 3;
        let results = Solution::k_smallest_pairs(nums1, nums2, k);
        assert_eq!(results, vec![
            vec![1,3],
            vec![2,3],
            vec![1,5],
        ]);
    }

}
