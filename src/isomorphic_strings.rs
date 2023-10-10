use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {

    // s and t are guaranteed to have the same length
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut result = true;

        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        let mut mappings = HashMap::new();
        let mut seen = HashSet::new();

        loop {
            match (s_chars.next(), t_chars.next()) {
                (Some(s_letter), Some(t_letter)) => {
                    if mappings.contains_key(&s_letter) {
                        result = mappings[&s_letter] == t_letter;
                        if !result {
                            break;
                        }
                    } else if seen.contains(&t_letter) {
                        result = false;
                        break;
                    } else {
                        seen.insert(t_letter);
                        mappings.insert(s_letter, t_letter);
                    }
                }
                _ => {
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
        let s = "egg".to_string();
        let t = "add".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(result);
    }

}
