/// Given an integer array `nums`, move all `0`'s to the end of it while
/// maintaining the relative order of the non-zero elements.
///
/// Note that you must do this in-place without making a copy of the array.
struct Solution;

impl Solution {

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut index = 0;

        for i in 0..n {
            let num = nums[i];
            if num != 0 {
                if index != i {
                    nums.swap(index, i);
                }
                index += 1;
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

}
