use std::collections::HashMap;

/// Given two strings `s` and `t`, return `true` if `t` is an
/// anagram of `s`, and `false` otherwise.
///
/// An anagram is a word or phrase formed by rearranging the letters of a
/// different word or phrase, typically using all the original letters exactly
/// once.
struct Solution;

impl Solution {
    fn add_letters(s: String) -> HashMap<char, i32> {
        let mut letters = HashMap::new();
        for c in s.chars() {
            letters
                .entry(c)
                .and_modify(|c| {
                    *c += 1;
                })
                .or_insert(1);
        }
        letters
    }

    fn remove_letters(letters: &mut HashMap<char, i32>, t: String) -> bool {
        let mut result = true;

        for c in t.chars() {
            let value = letters.get(&c);
            if value.is_none() {
                result = false;
                break;
            } else {
                let mut count = *value.unwrap();
                count -= 1;
                if count == 0 {
                    letters.remove(&c);
                } else {
                    letters.insert(c, count);
                }
            }
        }

        if result {
            result = letters.len() == 0;
        }

        result
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut letters = Self::add_letters(s);
        let result = Self::remove_letters(&mut letters, t);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }
}
