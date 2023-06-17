use std::collections::HashMap;

/// Given two strings `s1` and `s2`, return `true` if `s2` contains a permutation of `s1`, or
/// `false` otherwise.
///
/// In other words, return `true` if one of `s1`'s permutations is the substring of `s2`.
struct Solution;

impl Solution {

    fn counts(s1: String) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        for letter in s1.chars() {
            result
                .entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        result
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();
        let s2_len = s2.len();

        if s2_len < s1_len {
            false
        } else {
            let mut result = false;

            let s1_counts = Self::counts(s1);
            let s2_letters: Vec<char> = s2.chars().collect();
            let mut s2_counts = HashMap::new();

            for i in 0..s1_len {
                let letter = s2_letters[i];
                s2_counts
                    .entry(letter)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            let mut right = s1_len - 1;
            loop {
                if s1_counts == s2_counts {
                    result = true;
                    break;
                } else if right == s2_len - 1 {
                    break;
                } else {
                    let left = right + 1 - s1_len;
                    let letter = s2_letters[left];
                    s2_counts
                        .entry(letter)
                        .and_modify(|count| *count -= 1);
                    if s2_counts[&letter] == 0 {
                        s2_counts.remove(&letter);
                    }

                    right += 1;
                    let letter = s2_letters[right];
                    s2_counts
                        .entry(letter)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let result = Solution::check_inclusion(s1, s2);
        assert!(result);
    } 

    #[test]
    fn example_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let result = Solution::check_inclusion(s1, s2);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s1 = "abc".to_string();
        let s2 = "ccccbbbbaaaa".to_string();
        let result = Solution::check_inclusion(s1, s2);
        assert!(!result);
    }

}
