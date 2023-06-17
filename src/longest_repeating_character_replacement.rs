use std::collections::HashMap;

/// You are given a string `s` and an integer `k`. You can choose any character of the string and
/// change it to any other uppercase English character. You can perform this operation at most `k`
/// times.
///
/// Return the length of the longest substring containing the same letter you can get after
/// performing the above operations.
struct Solution;

impl Solution {

    fn is_valid(counts: &HashMap<char, usize>, left: usize, right: usize, k: i32) -> bool {
        let max = Self::max(counts) as i32;
        let length = (right - left + 1) as i32;
        length - max <= k
    }

    fn max(counts: &HashMap<char, usize>) -> usize {
        counts.values().max().copied().unwrap_or_default()
    }

    fn increment(counts: &mut HashMap<char, usize>, letter: char) {
        counts
            .entry(letter)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    fn decrement(counts: &mut HashMap<char, usize>, letter: char) {
        counts
            .entry(letter)
            .and_modify(|c| *c -= 1);
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut counts: HashMap::<char, usize> = HashMap::new();

        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();

        let mut result = 0;
        let mut left = 0;
        for right in 0..n {
            let letter = letters[right];
            Self::increment(&mut counts, letter);

            while !Self::is_valid(&counts, left, right, k) {
                Self::decrement(&mut counts, letters[left]);
                left += 1;
            }
            result = result.max((right - left + 1) as i32);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "ABAB".to_string();
        let result = Solution::character_replacement(s, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let s = "AABABBA".to_string();
        let result = Solution::character_replacement(s, 1);
        assert_eq!(result, 4);
    }

}
