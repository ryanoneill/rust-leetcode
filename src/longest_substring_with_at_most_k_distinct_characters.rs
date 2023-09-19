use std::collections::HashMap;

/// Given a string `s` and an integer `k`, return the length of the longest substring of `s` that
/// contains at most `k` distinct characters.
struct Solution;

impl Solution {

    fn add_letter(counts: &mut HashMap<char, usize>, letter: char) {
        counts
            .entry(letter)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    fn remove_letter(counts: &mut HashMap<char, usize>, letter: char) {
        if counts.contains_key(&letter) {
            let value = counts[&letter] - 1;
            if value == 0 {
                counts.remove(&letter);
            } else {
                counts.insert(letter, value);
            }
        }
    }

    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;

        if k > 0 {
            let letters: Vec<char> = s.chars().collect();
            let n = letters.len();

            let mut counts: HashMap<char, usize> = HashMap::new();
            let mut left = 0;

            for right in 0..n {
                let letter = letters[right];
                Self::add_letter(&mut counts, letter);

                while counts.keys().len() > k {
                    let old = letters[left];
                    Self::remove_letter(&mut counts, old);
                    left += 1;
                }
                let span = right - left + 1;
                result = result.max(span);
            }
        }

        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "eceba".to_string();
        let k = 2;
        let result = Solution::length_of_longest_substring_k_distinct(s, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = "aa".to_string();
        let k = 1;
        let result = Solution::length_of_longest_substring_k_distinct(s, k);
        assert_eq!(result, 2);
    }

}
