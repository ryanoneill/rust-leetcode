/// Given an integer array `nums` sorted in non-decreasing order, remove some
/// duplicates in-place such that each unique element appears at most twice.
/// The relative order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array in some languages,
/// you must instead have the result be placed in the first part of the array
/// `nums`. More formally, if there are `k` elements after removing the
/// duplicates, then the first `k` elements of `nums` should hold the final
/// result. It does not matter what you leave beyond the first `k` elements.
///
/// Return `k` after placing the final result in the first `k` slots of `nums`.
///
/// Do not allocate extra space for another array. You must do this by
/// modifying the input array in-place with O(1) extra memory.
struct Solution;

impl Solution {

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { 0 }
        else {
            let mut last = nums[0];
            let mut count = 1;
            let mut i = 1;

            for j in 1..n {
                let num = nums[j];
                if num == last {
                    count += 1;
                    if count <= 2 {
                        nums[i] = num;
                        i += 1;
                    }
                } else {
                    last = num;
                    nums[i] = num;
                    count = 1;
                    i += 1;
                }
            }

            i as i32
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1,1,1,2,2,3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        let _ = nums.split_off(5);
        assert_eq!(nums, vec![1,1,2,2,3]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0,0,1,1,1,1,2,3,3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 7);
        let _ = nums.split_off(7);
        assert_eq!(nums, vec![0,0,1,1,2,3,3]);
    }

}
