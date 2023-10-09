use std::collections::HashSet;

/// Given a string `s`, find the length of the longest substring without repeating
/// characters.
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let letters: Vec<char> = s.chars().collect();
        let mut items: HashSet<char> = HashSet::new();

        let mut result = 0;
        let mut current = 0;
        let mut left: usize = 0;

        for letter in letters.iter() {
            if items.contains(&letter) {
                while letters[left] != *letter {
                    items.remove(&letters[left]);
                    current -= 1;
                    left += 1;
                }
                left += 1;
            } else {
                items.insert(*letter);
                current += 1;
                result = result.max(current);
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
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let result = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(result, 3);
    }
}
