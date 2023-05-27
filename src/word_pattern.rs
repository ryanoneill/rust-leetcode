use std::collections::HashMap;
use std::collections::HashSet;

/// Given a `pattern` and a string `s`, find if `s` follows the
/// same pattern.
///
/// Here follow means a full match, such that there is a bijection between a
/// letter in pattern and a non-empty word in `s`.
struct Solution;

impl Solution {

    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut result = true;
        let mut mappings = HashMap::new();
        let mut used = HashSet::new();

        let mut letters = pattern.chars();
        let mut words = s.split_whitespace();

        loop {
            match (letters.next(), words.next()) {
                (Some(letter), Some(word)) => {
                    if mappings.contains_key(&letter) {
                        if mappings[&letter] != word {
                            result = false;
                            break;
                        }
                    } else if !used.contains(&word) {
                        mappings.insert(letter, word);
                        used.insert(word);
                    } else {
                        result = false;
                        break;
                    }
                }
                (None, None) => break,
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
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert!(!result);
    }

    #[test]
    fn reused() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        let result = Solution::word_pattern(pattern, s);
        assert!(!result);
    }

}
