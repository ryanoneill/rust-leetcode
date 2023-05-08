/// Given an input string `s` and a pattern `p`, implement regular expression
/// matching with support for `'.'` and '*' where:
///
/// * '.' Matches any single character.
///
/// * '*' Matches zero or more of the preceding element.
///
/// The matching should cover the entire input string (not partial).
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn is_match(_s: String, _p: String) -> bool {
        false
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let s = "aa".to_string();
        let p = "a".to_string();
        let result = Solution::is_match(s, p);
        assert!(!result);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let s = "aa".to_string();
        let p = "a*".to_string();
        let result = Solution::is_match(s, p);
        assert!(result);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let s = "ab".to_string();
        let p = ".*".to_string();
        let result = Solution::is_match(s, p);
        assert!(result);
    }

}
