/// Given an integer array `nums` and an integer `val`, remove all occurrences
/// of `val` in `nums` in-place. The order of the elements may be changed. Then
/// return the number of elements in `nums` which are not equal to `val`.
///
/// Consider the number of elements in `nums` which are not equal to `val` be
/// `k`, to get accepted, you need to do the following things:
///
/// * Change the array `nums` such that the first `k` elements of `nums`
///   contain the elements which are not equal to `val`. The remaining elements
///   of `nums` are not important as well as the size of `nums`.
///
/// * Return `k`.
struct Solution;

impl Solution {

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            let num = nums[j];
            if num != val {
                if i != j {
                    nums.swap(i, j);
                }
                i += 1;
            }
        }

        i as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![3,2,2,3];
        let val = 3;
        let result = Solution::remove_element(&mut nums, val);
        assert_eq!(result, 2);
        let _ = nums.split_off(2);
        assert_eq!(nums, vec![2,2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let result = Solution::remove_element(&mut nums, val);
        assert_eq!(result, 5);
        let _ = nums.split_off(5);
        assert_eq!(nums, vec![0,1,3,0,4]);
    }

}
