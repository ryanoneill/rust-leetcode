use std::collections::HashMap;
use std::collections::VecDeque;

/// Given a 0-indexed string `s`, permute `s` to get a new string `t` such that:
///
/// * All consonants remain in their original places. More formally, if there is an index `i` with
///   `0 <= i < s.length` such that `s[i]` is a consonant, the `t[i] = s[i]`.
///
/// * The vowels must be sorted in the nondecreasing order of their ASCII values. More formally,
///   for pairs of indices `i`, `j` with `0 <= i < j < s.length` such that `s[i]` and `s[j]` are
///   vowels, then `t[i]` must not have a higher ASCII value than `t[j]`.
///
/// Return the resulting string.
///
/// The vowels are `a`, `e`, `i`, `o`, and `u`, and they can appear in lowercase or uppercase.
/// Consonants comprise all letters that are not vowels.
struct Solution;

impl Solution {

    const VOWELS: &'static str = "AEIOUaeiou"; 

    pub fn sort_vowels(s: String) -> String {
        let mut letters: Vec<char> = s.chars().collect();
        let n = letters.len();

        let mut indexes = VecDeque::new();
        let mut counts = HashMap::new();

        for i in 0..n {
            let letter = letters[i];
            if Self::VOWELS.contains(letter) {
                indexes.push_back(i);
                counts
                    .entry(letter)
                    .and_modify(|c| { *c += 1 })
                    .or_insert(1);
            }
        }

        for key in Self::VOWELS.chars() {
            if counts.contains_key(&key) {
                let count = counts[&key];
                for _ in 0..count {
                    let index = indexes.pop_front().unwrap();
                    letters[index] = key;
                }
            }
        }

        letters.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("lEetcOde");
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "lEOtcede")
    }

    #[test]
    fn example_2() {
        let s = str!("lYmpH");
        let result = Solution::sort_vowels(s);
        assert_eq!(result, "lYmpH");
    }

}
