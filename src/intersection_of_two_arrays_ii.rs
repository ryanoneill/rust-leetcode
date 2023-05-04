use std::collections::HashMap;
use std::collections::HashSet;

/// Given two integer arrays `nums1` and `nums2`, return an array of their
/// intersection. Each element in the result must appear as many times as
/// it shows in both arrays and you may return the result in any order.
struct Solution;

impl Solution {
    fn to_counts(nums: Vec<i32>) -> HashMap<i32, i32> {
        let mut result = HashMap::new();
        for num in nums {
            result
                .entry(num)
                .and_modify(|c| {
                    *c += 1;
                })
                .or_insert(1);
        }
        result
    }

    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let map1 = Self::to_counts(nums1);
        let map2 = Self::to_counts(nums2);
        let keys1: HashSet<i32> = HashSet::from_iter(map1.keys().copied());
        let keys2: HashSet<i32> = HashSet::from_iter(map2.keys().copied());

        let mut result = Vec::new();
        for key in keys1.union(&keys2) {
            let val1 = map1.get(key).copied().unwrap_or_default();
            let val2 = map2.get(key).copied().unwrap_or_default();
            let min_val = val1.min(val2);
            for _ in 0..min_val {
                result.push(*key);
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
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let mut result = Solution::intersect(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let mut result = Solution::intersect(nums1, nums2);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
