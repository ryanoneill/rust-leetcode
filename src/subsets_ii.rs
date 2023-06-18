/// Given an integer array `nums` that may contain duplicates, return all possible subsets (the
/// power set).
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
            let mut cloned = current.clone();
            cloned.push(num);
            Self::backtrack(results, nums, cloned, i+1);

            let mut j = i;
            loop {
                if j == n {
                    break;
                } else if nums[j] == num {
                    j += 1;
                } else {
                    break;
                }
            }
            Self::backtrack(results, nums, current, j);
        }
    }


    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut results = Vec::new();
        let current = vec![];
        Self::backtrack(&mut results, &nums, current, 0);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,2];
        let mut result = Solution::subsets_with_dup(nums);
        result.sort();
        let mut expected = vec![vec![], vec![1], vec![1,2], vec![1,2,2], vec![2], vec![2,2]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![0];
        let mut result = Solution::subsets_with_dup(nums);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }

}
