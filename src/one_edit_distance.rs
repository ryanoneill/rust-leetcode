/// Given two strings `s` and `t`, return `true` if they are both one edit distance apart,
/// otherwise return `false`.
///
/// A string `s` is said to be one distance apart from a string `t` if you can:
///
/// * Insert exactly one character into `s` to get `t`.
///
/// * Delete exactly one character from `s` to get `t`.
///
/// * Replace exactly one character of `s` with a different character to get `t`.
struct Solution;

impl Solution {

    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let m = s_chars.len();
        let n = t_chars.len();
        let diff = m.max(n) - m.min(n);

        let mut result = true;
        if diff > 1 {
            result = false;
        } else {
            let mut count = 0;

            let mut i = 0;
            let mut j = 0;

            while i < m || j < n {
                let s_letter = if i < m {
                    s_chars[i]
                } else {
                    ' '
                };
                let t_letter = if j < n {
                    t_chars[j]
                } else {
                    ' '
                };

                if s_letter == t_letter {
                    i += 1;
                    j += 1;
                } else {
                    count += 1;
                    if m == n {
                        i += 1;
                        j += 1;
                    } else if m < n {
                        j += 1;
                    } else {
                        i += 1;
                    }

                    if count > 1 {
                        result = false;
                        break;
                    }
                }
            }
            result = result && count == 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "ab".to_string();
        let t = "acb".to_string();
        let result = Solution::is_one_edit_distance(s, t);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "".to_string();
        let t = "".to_string();
        let result = Solution::is_one_edit_distance(s, t);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s = "a".to_string();
        let t = "".to_string();
        let result = Solution::is_one_edit_distance(s, t);
        assert!(result);
    }

}
