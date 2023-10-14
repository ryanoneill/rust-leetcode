/// Given an integer array `nums`, reorder it such that `nums[0] <= nums[1] >= nums[2] <=
/// nums[3]...`.
///
/// You may assume the input array always has a valid answer.
struct Solution;

impl Solution {

    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut less_than_or_equal = true;

        for i in 1..n {
            let previous = nums[i-1];
            let current = nums[i];

            if less_than_or_equal {
                if previous > current {
                    nums.swap(i-1, i);
                }
            } else {
                if current > previous {
                    nums.swap(i-1, i);
                }
            }
            less_than_or_equal = !less_than_or_equal;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![3,5,2,1,6,4];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![3,5,1,6,2,4]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![6,6,5,6,3,8];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![6,6,5,6,3,8]);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn example_4() {
        let mut nums = vec![1,2,3,4,5,6,7];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![1,3,2,5,4,7,6]);
    }

    #[test]
    fn example_5() {
        let mut nums = vec![9,8,7,6,5,4];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![8,9,6,7,4,5]);
    }

}
