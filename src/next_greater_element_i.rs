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

    fn has_lesser(nums: &Vec<i32>, stack: &Vec<usize>, num: i32) -> bool {
        if stack.is_empty() { false }
        else {
            let last_index = stack.iter().last().unwrap();
            let last_num = nums[*last_index];
            last_num < num
        }
    }

    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len());
        let mut stack = vec![];
        let mut greater = HashMap::new();

        for (i, num2) in nums2.iter().enumerate() {
            while Self::has_lesser(&nums2, &stack, *num2) {
                let j: usize = stack.pop().unwrap();
                greater.insert(nums2[j], num2);
            }
            stack.push(i);
        }

        for num1 in nums1.iter() {
            match greater.get(num1) {
                Some(&g) => { result.push(*g); }
                None => { result.push(-1); }
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
