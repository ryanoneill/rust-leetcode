use crate::stack::Stack;

/// Given a string `s` containing just the characters '(', ')', '{', '}', '['
/// and ']', determine if the input string is valid.
///
/// An input string is valid if:
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
/// - Every close bracket has a corresponding open bracket of the same type.
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Stack::new();
        let mut result = true;

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                ')' | ']' | '}' => {
                    match stack.pop() {
                        Some('(') if c == ')' => {} // all good
                        Some('[') if c == ']' => {} // all good
                        Some('{') if c == '}' => {} // all good
                        _ => {
                            // No character, or not the right match
                            result = false;
                            break;
                        }
                    }
                }
                _ => {
                    // Unexpected character
                    result = false;
                    break;
                }
            }
        }
        // If still good, verify that the stack is empty
        result && stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "()".to_string();
        let result = Solution::is_valid(s);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "()[]{}".to_string();
        let result = Solution::is_valid(s);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let s = "(]".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
    }

    #[test]
    fn nested() {
        let s = "([{}])".to_string();
        let result = Solution::is_valid(s);
        assert!(result);
    }

    #[test]
    fn same_matched() {
        let s = "{{{{{}}}}}".to_string();
        let result = Solution::is_valid(s);
        assert!(result);
    }

    #[test]
    fn same_unmatched_open() {
        let s = "{{{{{}}}}".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
    }

    #[test]
    fn same_unmatched_close() {
        let s = "{{{{}}}}}".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
    }
}
