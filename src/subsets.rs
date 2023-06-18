/// Given an integer array `nums` of unique elements, return all possible subsets (the power set).
///
/// The solution set must not contain duplicate subsets. Return the solution in any order.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<Vec<i32>>, nums: &Vec<i32>, current: Vec<i32>, i: usize) {
        let n = nums.len();

        if i == n {
            results.push(current);
        } else {
            let num = nums[i];
            let mut current = current;
            let cloned = current.clone();
            Self::backtrack(results, nums, cloned, i + 1);
            current.push(num);
            Self::backtrack(results, nums, current, i + 1);
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let capacity = 2_u32.pow(n as u32) as usize;

        let mut results = Vec::with_capacity(capacity);
        Self::backtrack(&mut results, &nums, Vec::new(), 0);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3];
        let mut result = Solution::subsets(nums);
        result.sort();
        let mut expected = vec![vec![], vec![1], vec![2], vec![1,2], vec![3], vec![1,3], vec![2,3], vec![1,2,3]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![0];
        let result = Solution::subsets(nums);
        assert_eq!(result, vec![vec![], vec![0]]);
    }

}
