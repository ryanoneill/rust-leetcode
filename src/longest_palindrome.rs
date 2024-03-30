use std::collections::HashMap;

/// Given a string `s` which consists of lowercase or uppercase letters, return the length of the
/// longest palindrome that can be built with those letters.
struct Solution;

impl Solution {

    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts = HashMap::new();
        for letter in s.chars() {
            counts
                .entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut result = 0;
        for (_, count) in counts {
            if count % 2 == 0 {
                result += count;
            } else if result % 2 == 0 {
                result += count;
            } else {
                result += count - 1;
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
        let s = "abccccdd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 1)
    }

}
