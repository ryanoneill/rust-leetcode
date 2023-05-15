use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {

    // Range is -10000 through 10000
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = vec![0; 20001];
        for num in nums {
            let index = num as usize + 10000;
            counts[index] += 1;
        }

        let mut result = -10000;
        let mut left = k;

        let mut i = 20000;
        while left > 0 {
            let count = counts[i];
            left -= count;
            if left <= 0 {
                result = i as i32 - 10000;
            } else if i == 0 {
                break;
            } else {
                i -= 1;
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
