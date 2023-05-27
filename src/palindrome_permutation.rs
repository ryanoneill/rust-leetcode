use std::collections::HashMap;

/// Given a string `s`, return `true` if a permutation of the string could form
/// a palindrome and `false` otherwise.
struct Solution;

impl Solution {

    pub fn can_permute_palindrome(s: String) -> bool {
        let mut counts = HashMap::new();
        for letter in s.chars() {
            counts
                .entry(letter)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut odd_counts = 0;
        for (_, count) in counts.iter() {
            if count % 2 == 1 {
                odd_counts += 1;
            }
        }

        odd_counts <= 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "code".to_string();
        let result = Solution::can_permute_palindrome(s);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let s = "aab".to_string();
        let result = Solution::can_permute_palindrome(s);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let s = "carerac".to_string();
        let result = Solution::can_permute_palindrome(s);
        assert!(result);
    }

}
