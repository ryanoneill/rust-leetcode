/// A phrase is a palindrome if, after converting all uppercase letters into
/// lowercase letters and removing all non-alphanumeric characters, it reads
/// the same forward and backward. Alphanumeric characters include letters and
/// numbers.
///
/// Given a string `s`, return `true` if it is a palindrome, or `false`
/// otherwise.
struct Solution;

impl Solution {

    pub fn is_palindrome(s: String) -> bool {
        let mut result = true;
        let chars: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            let mut left_char = chars[left];
            let mut right_char = chars[right];

            if !left_char.is_ascii_alphanumeric() {
                left += 1;
            } else if !right_char.is_ascii_alphanumeric() {
                right -= 1;
            } else {
                left_char = left_char.to_ascii_lowercase();
                right_char = right_char.to_ascii_lowercase();
                if left_char != right_char {
                    result = false;
                    break;
                } else {
                    left += 1;
                    right -= 1;
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
        let s = "A man, a plan, a canal: Panama".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "race a car".to_string();
        let result = Solution::is_palindrome(s);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s = " ".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }

    #[test]
    fn numbers() {
        let s = "0P".to_string();
        let result = Solution::is_palindrome(s);
        assert!(!result);
    }

}
