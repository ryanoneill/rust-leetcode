/// Given an array of integers `nums`, sort the array in ascending order and
/// return it.
///
/// You must solve the problem without using any built-in functions in
/// `O(n log(n))` time complexity and with the smallest space complexity
/// possible.
struct Solution;

impl Solution {

    fn fix_bottom_up(nums: &mut Vec<i32>, n: usize, i: usize) {
        if i != 0 {
            let parent = (i - 1) / 2;

            if nums[parent] < nums[i] {
                nums.swap(i, parent);
                if parent != 0 {
                    Self::fix_bottom_up(nums, n, parent);
                }
            }
        }
    }

    fn fix_top_down(nums: &mut Vec<i32>, n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && nums[left] > nums[largest] {
            largest = left;
        }

        if right < n && nums[right] > nums[largest] {
            largest = right;
        }

        if largest != i {
            nums.swap(i, largest);
            Self::fix_top_down(nums, n, largest);
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;

        for i in 0..n {
            Self::fix_bottom_up(&mut nums, i+1, i);
        }

        for i in 0..n {
            let j = n - i - 1;
            nums.swap(0, j);
            Self::fix_top_down(&mut nums, j, 0);
        }

        nums
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![5,2,3,1];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![1,2,3,5]);
    }

    #[test]
    fn example_2() {
        let nums = vec![5,1,1,2,0,0];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![0,0,1,1,2,5]);
    }

    #[test]
    fn empty() {
        let nums = vec![];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn one() {
        let nums = vec![37];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![37]);
    }

    #[test]
    fn two() {
        let nums = vec![2,1];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![1,2]);
    }

}
