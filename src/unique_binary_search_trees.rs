/// Given an integer `n`, return the number of structurally unique BST's (binary search trees)
/// which has exactly `n` nodes of unique values from `1` to `n`.
struct Solution;

impl Solution {

    // https://en.wikipedia.org/wiki/Catalan_number
    pub fn num_trees(n: i32) -> i32 {
        let n = n as i64;
        let mut result: i64 = 1;
        for i in 0..n {
            result = result * 2 * (2 * i + 1) / (i + 2);
        }
        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let result = Solution::num_trees(n);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::num_trees(n);
        assert_eq!(result, 1);
    }

}
