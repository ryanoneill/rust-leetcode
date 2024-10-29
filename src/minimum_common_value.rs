struct Solution;

impl Solution {

    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        let mut i1 = 0;
        let mut i2 = 0;

        let mut result = -1;

        loop {
            if i1 == n1 || i2 == n2 {
                break;
            }
            if nums1[i1] == nums2[i2] {
                result = nums1[i1];
                break;
            } else if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
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
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![1,2,3,6];
        let nums2 = vec![2,3,4,5];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let nums1 = vec![];
        let nums2 = vec![];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, -1);
    }


}
