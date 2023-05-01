/// Given a string `s`, return the longest palindromic substring in s.
pub struct Solution;

impl Solution {
    // TODO: Implement
    pub fn longest_palindrome(_s: String) -> String {
        // let letters: Vec<char> = s.chars().collect();

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert_eq!(result, "bab");
    }

    #[ignore]
    #[test]
    fn example_2() {
        let result = Solution::longest_palindrome("cbbd".to_string());
        assert_eq!(result, "bb");
    }
}
