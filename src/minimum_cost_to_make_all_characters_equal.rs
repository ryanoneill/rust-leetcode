/// You are given a 0-indexed binary string `s` of length `n` on which you can
/// apply two types of operations:
///
/// * Choose an index `i` and invert all characters from index `0` to `i` (both
///   inclusive), with a cost of `i + 1`
///
/// * Choose an index `i` and invert all characters from index `i` to index
///   `n - 1` (both inclusive), with a cost of `n - i`
///
/// Return the minimum cost to make all characters of the string equal.
///
/// Invert a character means if its values is '0' it becomes '1' and vice-versa.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn minimum_cost(_s: String) -> i64 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let s = "0011".to_string();
        let result = Solution::minimum_cost(s);
        assert_eq!(result, 2);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let s = "010101".to_string();
        let result = Solution::minimum_cost(s);
        assert_eq!(result, 9);
    }

}
