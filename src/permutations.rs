use std::collections::HashSet;

/// Given an array `nums` of distinct integers, return all the possible permutations. You can
/// return the answer in any order.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<Vec<i32>>, nums: &Vec<i32>, current: Vec<i32>, seen: HashSet<i32>) {
        let n = nums.len();
        let c = current.len();

        if n == c {
            results.push(current);
        } else {
            for num in nums {
                if !seen.contains(&num) {
                    let mut new_current = current.clone();
                    new_current.push(*num);
                    let mut new_seen = seen.clone();
                    new_seen.insert(*num);

                    Self::backtrack(results, nums, new_current, new_seen);
                }
            }

        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let current = Vec::new();
        let seen = HashSet::new();
        Self::backtrack(&mut results, &nums, current, seen);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[test]
    fn example_1() {
        let nums = vec![1,2,3];
        let result = Solution::permute(nums);
        assert_eq!(result, vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,1];
        let result = Solution::permute(nums);
        assert_eq!(result, vec![vec![0,1], vec![1,0]]);
    }

    #[test]
    fn example_3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        assert_eq!(result, vec![vec![1]]);
    }

}
