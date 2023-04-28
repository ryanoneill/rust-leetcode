use std::collections::HashMap;
use std::collections::HashSet;

/// Given two strings `ransomNote` and `magazine`, return `true` if
/// `ransomNote` can be constructed by using the letters from `magazine` and
/// `false` otherwise.
///
/// Each letter in `magazine` can only be used once in `ransomNote`.
struct Solution;

impl Solution {

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let needed: HashSet<char> = HashSet::from_iter(ransom_note.chars());
        let mut letter_counts: HashMap<char, usize> = HashMap::new();

        for letter in magazine.chars() {
            if needed.contains(&letter) {
                letter_counts
                    .entry(letter)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let mut result = true;
        for letter in ransom_note.chars() {
            match letter_counts.get_mut(&letter) {
                Some(value) if *value > 1 => {
                    *value -= 1;
                }
                Some(value) if *value == 1 => {
                    letter_counts.remove(&letter);
                }
                _ => {
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
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        let result = Solution::can_construct(ransom_note, magazine);
        assert!(result);
    }

}
