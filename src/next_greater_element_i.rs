use std::collections::HashMap;

/// The next greater element of some element `x` in an array is the first
/// greater element that is to the right of `x` in the same array.
///
/// You are given two distinct 0-indexed integer arrays `nums1` and `nums2`,
/// where `nums1` is a subset of `nums2`.
///
/// For each `0 <= i < nums1.length`, find the `j` such that
/// `nums1[i] == nums2[j]` and determine the next greater element of `nums2[j]`
/// in `nums2`. If there is no next greater element, then the answer for this
/// query is -1.
///
/// Return an array `ans` of length `nums1.length` such that `ans[i]` is the
/// next greater element as described above.
struct Solution;

impl Solution {

    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();

        let n = nums2.len();
        for i in 0..n {
            let num2 = nums2[i];

            loop {
                if stack.is_empty() {
                    break;
                } else {
                    let s = stack.len();
                    let peek = stack[s-1];
                    if num2 > peek {
                        map.insert(peek, num2);
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }
            stack.push(num2);
        }

        let m = nums1.len();
        let mut result: Vec<i32> = vec![-1; m];

        for i in 0..m {
            let num1 = nums1[i];
            if map.contains_key(&num1) {
                result[i] = map[&num1];
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
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let result = Solution::next_greater_element(nums1, nums2);
        assert_eq!(result, vec![-1, 3, -1]);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let result = Solution::next_greater_element(nums1, nums2);
        assert_eq!(result, vec![3, -1]);
    }
}
