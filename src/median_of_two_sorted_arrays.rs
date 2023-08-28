/// Given two sorted arrays `nums` and `nums2` of size `m` and `n`
/// respectively, return the median of the two sorted arrays.
///
/// The overall run time complexity should be O(log (m+n)).
pub struct Solution;

impl Solution {

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();

        if m > n {
            Self::find_median_sorted_arrays(nums2, nums1)
        } else {
            let mut result = 0.0;

            let total = m + n;
            let half = total / 2;

            let mut left = 0;
            let mut right = m;

            while left <= right {
                let mid = left + (right - left) / 2;
                let rest = half - mid;

                let m_left = if mid > 0 {
                    nums1[mid - 1] as f64
                } else {
                    f64::MIN
                };

                let m_right = if mid < m {
                    nums1[mid] as f64
                } else {
                    f64::MAX
                };

                let n_left = if rest > 0 {
                    nums2[rest - 1] as f64
                } else {
                    f64::MIN
                };

                let n_right = if rest < n {
                    nums2[rest] as f64
                } else {
                    f64::MAX
                };

                if m_left <= n_right && n_left <= m_right {
                    // Partitioned Correctly
                    if total % 2 == 1 {
                        result = m_right.min(n_right);
                    } else {
                        let left_max = m_left.max(n_left);
                        let right_min = m_right.min(n_right);
                        result = (left_max + right_min) / 2.0;
                    }
                    break;
                } else if m_left > n_right {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn example_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);
    }
}
