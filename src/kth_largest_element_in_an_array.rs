use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {

    fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot_index: usize) -> usize {
        let pivot = nums[pivot_index];
        nums.swap(pivot_index, right);
        let mut store_index = left;

        for i in left..right {
            let value = nums[i];
            if value < pivot {
                nums.swap(store_index, i);
                store_index += 1;
            }
        }
        nums.swap(right, store_index);
        store_index
    }

    fn select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
        if left == right { 
            nums[left]
        } else {
            // This should be random but ...
            let mut pivot_index = left + (right - left) / 2;
            pivot_index = Self::partition(nums, left, right, pivot_index);
            if pivot_index == k {
                nums[pivot_index]
            } else if k < pivot_index {
                Self::select(nums, left, pivot_index - 1, k)
            } else {
                Self::select(nums, pivot_index + 1, right, k)
            }
        }
    }

    // Modified to use quickselect. Last implementation used counting sort
    // which according to the benchmarks was faster.
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let k = k as usize;
        Self::select(&mut nums, 0, n-1, n - k)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 4);
    }

}
