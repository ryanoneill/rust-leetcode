use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

/// You are given an array `nums` consisting of positive integers.
///
/// Starting at `score = 0`, apply the following algorithm:
///
/// * Choose the smallest integer of the array that is not marked. If there is a tie, choose the
/// one with the smallest index.
///
/// * Add the value of the chosen integer to `score`.
///
/// * Mark the chosen element and its two adjacent elements if they exist.
///
/// * Repeat until all the array elements are marked.
///
/// Return the score you get after applying the above algorithm.
struct Solution;

impl Solution {

    pub fn find_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut marked: HashSet<usize> = HashSet::new();
        let mut mins: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut sum = 0;

        for i in 0..n {
            let item = (nums[i], i);
            mins.push(Reverse(item));
        }

        while !mins.is_empty() && marked.len() < n {
            let current = mins.pop().unwrap().0;
            let value = current.0;
            let index = current.1;
            if !marked.contains(&index) {
                marked.insert(index);
                sum += value as i64;

                if index > 0 {
                    marked.insert(index - 1);
                }
                if index < n-1 {
                    marked.insert(index + 1);
                }
            } 
        }

        sum
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2, 1, 3, 4, 5, 2];
        let result = Solution::find_score(nums);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,3,5,1,3,2];
        let result = Solution::find_score(nums);
        assert_eq!(result, 5);
    }

}
