/// Find all valid combinations of `k` numbers that sum up to `n` such that the following
/// conditions are true:
///
/// * Only numbers `1` through `9` are used.
///
/// * Each number is used at most once.
///
/// Return a list of all possible valid combinations. The list must not contain the same
/// combination twice, and the combinations may be returned in any order.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<Vec<i32>>, k: usize, n: i32, current: Vec<i32>, current_sum: i32, i: usize) {
        if current.len() == k && current_sum == n {
            results.push(current);
        } else if current_sum > n || current.len() > k {
            // Do Nothing
        } else {
            for j in i..=9 {
                let mut cloned = current.clone();
                cloned.push(j as i32);
                let cloned_sum = current_sum + (j as i32);
                Self::backtrack(results, k, n, cloned, cloned_sum, j+1);
            }
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut results = vec![];
        let current = vec![];
        Self::backtrack(&mut results, k, n, current, 0, 1);
        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let k = 3;
        let n = 7;
        let result = Solution::combination_sum3(k, n);
        assert_eq!(result, vec![vec![1,2,4]]);
    }

    #[test]
    fn example_2() {
        let k = 3;
        let n = 9;
        let mut result = Solution::combination_sum3(k, n);
        result.sort();
        assert_eq!(result, vec![vec![1,2,6], vec![1,3,5], vec![2,3,4]]);
    }

}
