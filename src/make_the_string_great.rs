use crate::stack::Stack;

/// Given a string `s` of lower and upper case English letters.
///
/// A good string is a string which doesn't have two adjacent characters `s[i]`
/// and `s[i + 1]` where:
/// * `0 <= i <= s.length - 2`
/// * `s[i]` is a lower case letter and `s[i + 1]` is the same letter but in
///   upper-case or vice-versa.
///
/// To make the string good, you can choose two adjacent characters that make
/// the string bad and remove them. You can keep doing this until the string
/// becomes good.
///
/// Return the string after making it good. The answer is guaranteed to be
/// unique under the given constraints.
///
/// Notice that an empty string is also good.
pub struct Solution;

impl Solution {

    fn is_opposite_case(first: char, second: char) -> bool {
        if first.is_ascii_lowercase() && second.is_ascii_uppercase() {
            first == second.to_ascii_lowercase()
        } else if first.is_ascii_uppercase() && second.is_ascii_lowercase() {
            first == second.to_ascii_uppercase()
        } else { false }
    }

    pub fn make_good(s: String) -> String {
        let mut stack = Stack::new();

        for c in s.chars() {
            match stack.peek() {
                Some(value) if Self::is_opposite_case(c, *value) => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        stack.to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "leEeetcode".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "leetcode");
    }

    #[test]
    fn example_2() {
        let s = "abBAcC".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "");
    }

    #[test]
    fn example_3() {
        let s = "s".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "s");
    }

    #[test]
    fn uppercase_first() {
        let s = "ABbaC".to_string();
        let result = Solution::make_good(s);
        assert_eq!(result, "C");
    }

}
