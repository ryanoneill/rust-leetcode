/// Given a positive integer `num` represented as a string, return the integer
/// `num` without trailing zeros as a string.
struct Solution;

impl Solution {

    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches(|d| d == '0').to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = "51230100".to_string();
        let result = Solution::remove_trailing_zeros(num);
        assert_eq!(result, "512301");
    }

    #[test]
    fn example_2() {
        let num = "123".to_string();
        let result = Solution::remove_trailing_zeros(num);
        assert_eq!(result, "123");
    }

}
