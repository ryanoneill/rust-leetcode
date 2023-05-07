struct Solution;

impl Solution {

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;

        if n == 0 { } // do nothing
        else if m == 0 {
            for i in 0..n {
                nums1[i] = nums2[i];
            }
        } else {
            let mut nums1_i = m-1;
            let mut nums2_i = n-1;
            let mut nums1_done = false;
            let mut nums2_done = false;

            let total = m + n;

            for i in 0..total {
                let index = total-i-1;
                if nums2_done {
                    nums1[index] = nums1[nums1_i];
                    if nums1_i == 0 { nums1_done = true; }
                    else { nums1_i -= 1; }
                } else if nums1_done {
                    nums1[index] = nums2[nums2_i];
                    if nums2_i == 0 { nums2_done = true; }
                    else { nums2_i -= 1; }
                } else {
                    let num1 = nums1[nums1_i];
                    let num2 = nums2[nums2_i];
                    if num2 > num1 {
                        nums1[index] = num2;
                        if nums2_i == 0 { nums2_done = true; }
                        else { nums2_i -= 1; }
                    } else {
                        nums1[index] = num1;
                        if nums1_i == 0 { nums1_done = true; }
                        else { nums1_i -= 1; }
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }

    #[test]
    fn example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

}
