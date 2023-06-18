/// Given two integers `n` and `k`, return all possible combinations of `k` numbers chosen from the
/// range `[1,n]`.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<Vec<i32>>, n: usize, k: usize, current: Vec<i32>, i: usize) {
        if current.len() == k {
            results.push(current);
        } else if i < n {
            let num = (i + 1) as i32;
            let mut current = current;
            let cloned = current.clone();
            Self::backtrack(results, n, k, cloned, i + 1);
            current.push(num);
            Self::backtrack(results, n, k, current, i + 1);
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let k = k as usize;

        let mut results = Vec::new();
        let current = vec![];
        let i = 0;
        Self::backtrack(&mut results, n, k, current, i);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let k = 2;
        let mut result = Solution::combine(n, k);
        result.sort();
        let mut expected = vec![vec![1,2], vec![1,3], vec![1,4], vec![2,3], vec![2,4], vec![3,4]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let k = 1;
        let result = Solution::combine(n, k);
        assert_eq!(result, vec![vec![1]]);
    }

}
