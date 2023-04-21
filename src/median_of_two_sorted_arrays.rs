/// Given two sorted arrays `nums` and `nums2` of size `m` and `n`
/// respectively, return the median of the two sorted arrays.
///
/// The overall run time complexity should be O(log (m+n)).
pub struct Solution;

impl Solution {

    // TODO: Implement
    pub fn find_median_sorted_arrays(_nums1: Vec<i32>, _nums2: Vec<i32>) -> f64 {
        0.0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore] #[test]
    fn example_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }

    #[ignore] #[test]
    fn example_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);
    }

}
