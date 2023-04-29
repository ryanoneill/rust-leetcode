/// Given two strings `s` and `t`, return `true` if `s` is a subsequence of
/// `t`, or `false` otherwise.
///
/// A subsequence of a string is a new string that is formed from the original
/// string by deleting some (can be none) of the characters without disturbing
/// the relative positions of the remaining characters. (i.e., `"ace"` is a
/// subsequence of `"abcde"` while `"aec"` is not).
struct Solution;

impl Solution {

    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        let mut s_current = s_chars.next();
        let mut t_current = t_chars.next();

        let mut result = true;

        loop {
            match (s_current, t_current) {
                (Some(s_candidate), Some(t_candidate)) => {
                    if s_candidate == t_candidate {
                        s_current = s_chars.next();
                    }
                    t_current = t_chars.next();
                }
                (None, _) => {
                    break;
                }
                (_, _) => {
                    result = false;
                    break;
                }
            }

        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let result = Solution::is_subsequence(s, t);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let result = Solution::is_subsequence(s, t);
        assert!(!result);
    }

}
