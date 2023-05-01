/// You are given a string `s`, which contains stars `*`.
///
/// In one operation, you can:
/// * Choose a star in `s`.
/// * Remove the closest non-star character to its left, as well as remove the
///   star itself.
///
/// Return the string after all stars have been removed.
///
/// Note:
/// * The input will be generated such that the operation is always possible.
/// * It can be shown that the resulting string will always be unique.
struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result: String = String::from("");

        for c in s.chars() {
            match c {
                '*' => {
                    if !result.is_empty() {
                        result.pop();
                    }
                }
                _ => result.push(c),
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
        let s = "leet**cod*e".to_string();
        let result = Solution::remove_stars(s);
        assert_eq!(result, "lecoe");
    }

    #[test]
    fn example_2() {
        let s = "erase*****".to_string();
        let result = Solution::remove_stars(s);
        assert_eq!(result, "");
    }
}
