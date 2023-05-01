/// Given an integer `x`, return `true` if `x` is a palindrome, and false
/// otherwise.
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let chars: Vec<char> = x.to_string().chars().collect();
        let len = chars.len();
        let mut result = true;

        for i in 0..(len / 2) {
            if chars[i] != chars[len - i - 1] {
                result = false;
                break;
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
        let x = 121;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let x = -121;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let x = 10;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }

    #[test]
    fn even_number_of_digits() {
        let x = 1234554321;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn odd_number_of_digits() {
        let x = 123454321;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn one_digit() {
        let x = 9;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }
}
