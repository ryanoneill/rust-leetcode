/// Given a string `s`, return `true` if the `s` can be palindrome after deleting at most one
/// character from it.
struct Solution;

impl Solution {

    fn is_palindrome(letters: &Vec<char>, left: usize, right: usize, count: usize) -> bool {
        if count > 1 {
            false
        } else if left >= right {
            true
        } else {
            let left_letter = letters[left];
            let right_letter = letters[right];

            if left_letter == right_letter {
                Self::is_palindrome(letters, left+1, right-1, count)
            } else {
                Self::is_palindrome(letters, left+1, right, count+1) ||
                    Self::is_palindrome(letters, left, right-1, count+1)
            }
        }
    }

    pub fn valid_palindrome(s: String) -> bool {
        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();
        let result = Self::is_palindrome(&letters, 0, n-1, 0);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("aba");
        let result = Solution::valid_palindrome(s);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = str!("abca");
        let result = Solution::valid_palindrome(s);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let s = str!("abc");
        let result = Solution::valid_palindrome(s);
        assert!(!result);
    }

}
