use crate::stack::Stack;

/// You are given a string `s` consisting of lowercase English letters. A
/// duplicate removal consists of choosing two adjacent and equal letters and
/// removing them.
///
/// We repeatedly make duplicate removals on `s` until we no longer can.
///
/// Return the final string after all such duplicate removals have been made.
/// It can be proven that the answer is unique.
struct Solution;

impl Solution {

    pub fn remove_duplicates(s: String) -> String {
        let mut stack = String::from("");
        for c in s.chars() {
            match stack.chars().last() {
                Some(v) if c == v => { stack.pop(); }
                _ => { stack.push(c); }
            }
        }
        stack
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abbaca".to_string();
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, "ca");
    }

    #[test]
    fn example_2() {
        let s = "azxxzy".to_string();
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, "ay");
    }

    #[test]
    fn all_removed() {
        let s = "abcdeedcba".to_string();
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, "");
    }

}
