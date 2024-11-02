use std::collections::HashMap;

/// There are `n` people standing in a queue, and they numbered from `0` to `n - 1` in left to
/// right order. You are given an array `heights` of distinct integers where `heights[i]`
/// represents the height of the `ith` person.
///
/// A person can see another person to their right in the queue if everybody in between is shorter
/// than both of them. More formally, the `ith` person can see the `jth` person if `i < j` and
/// `min(heights[i], heights[j]) > max(heights[i+1], heights[i+2], ..., heights[j-1])`.
///
/// Return an array `answer` of length `n` where `answer[i]` is the number of people the `ith`
/// person can see to their right in the queue.
struct Solution;

impl Solution {

    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for j in 0..n {
            let height = heights[j];
            loop {
                if stack.is_empty() {
                    break;
                } else {
                    let s = stack.len();
                    let i = stack[s-1]; 
                    let peek = heights[i];
                    result[i] += 1;
                    if height > peek {
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }
            stack.push(j);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![10, 6, 8, 5, 11, 9];
        let result = Solution::can_see_persons_count(nums);
        assert_eq!(result, [3, 1, 2, 1, 1, 0]);
    }

    #[test]
    fn example_2() {
        let nums = vec![5, 1, 2, 3, 10];
        let result = Solution::can_see_persons_count(nums);
        assert_eq!(result, vec![4,1,1,1,0]);
    }



}
