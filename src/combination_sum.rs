/// Given an array of distinct integers `candidates` and a target integer `target`, return a list
/// of all unique combinations of `candidates` where the chosen numbers sum to target. You may
/// return the combinations in any order. 
///
/// The same number may be chosen from `candidates` an unlimited number of times. Two combinations
/// are unique if the frequency of at least one of the chosen numbers is different.
///
/// The test cases are generated such that the number of unique combinations that sum up to
/// `target` is less than `150` combinations for the given input.
struct Solution;

impl Solution {

    // target is <= 40, so i32 should work for everything.
    fn backtrack(
        results: &mut Vec<Vec<i32>>,
        candidates: &Vec<i32>,
        target: i32,
        current: Vec<i32>,
        current_sum: i32,
        i: usize) {

        if current_sum > target {
            // Do Nothing, dead end
        } else if current_sum == target {
            results.push(current);
        } else {
            let n = candidates.len();
            for j in i..n {
                let num = candidates[j];
                let mut cloned = current.clone();
                cloned.push(num);
                let cloned_sum = current_sum + num;
                Self::backtrack(results, candidates, target, cloned, cloned_sum, j);
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let current = Vec::new();
        Self::backtrack(&mut results, &candidates, target, current, 0, 0);
        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let candidates = vec![2,3,6,7];
        let target = 7;
        let mut result = Solution::combination_sum(candidates, target);
        result.sort();
        assert_eq!(result, vec![vec![2,2,3], vec![7]]);
    }

    #[test]
    fn example_2() {
        let candidates = vec![2,3,5];
        let target = 8;
        let mut result = Solution::combination_sum(candidates, target);
        result.sort();
        assert_eq!(result, vec![vec![2,2,2,2], vec![2,3,3], vec![3,5]]);
    }

    #[test]
    fn example_3() {
        let candidates = vec![2];
        let target = 1;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

}
