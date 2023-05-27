/// Given a string `s`, return the longest palindromic substring in s.
pub struct Solution;

impl Solution {

    fn expand_out_from(letters: &Vec<char>, left: usize, right: usize) -> usize {
        let n = letters.len();
        let mut result = 0;

        let mut left = left;
        let mut right = right;

        loop {
            let left_letter = letters[left];
            let right_letter = letters[right];
            if left_letter == right_letter {
                if left == right { 
                    result += 1; 
                } else {
                    result += 2;
                }

                if left == 0 {
                    break;
                } else if right == n - 1 {
                    break;
                } else {
                    left -= 1;
                    right += 1;
                }
            } else {
                break;
            }
        }

        result
    }

    fn get_palindrome(letters: &Vec<char>, i: usize, len: usize) -> String {
        let start = i - (len - 1) / 2;
        let end = i + len / 2 + 1;
        letters[start..end].iter().collect()
    }

    pub fn longest_palindrome(s: String) -> String {
        let mut result = String::from("");
        let mut result_len = 0;

        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();

        for i in 0..n {
            let centered = Self::expand_out_from(&letters, i, i);
            if centered >= result_len {
                result = Self::get_palindrome(&letters, i, centered);
                result_len = result.len();
            }
            if i < n-1 {
                let between = Self::expand_out_from(&letters, i, i+1);
                if between >= result_len {
                    result = Self::get_palindrome(&letters, i, between);
                    result_len = result.len();
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
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert!(result == "bab" || result == "aba")
    }

    #[test]
    fn example_2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }

    #[test]
    fn longer() {
        let s = "hellohowareyoutodayracecarblueseriously".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "racecar");
    }

    #[test]
    fn single() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }

    #[test]
    fn double() {
        let s = "bb".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }

}
