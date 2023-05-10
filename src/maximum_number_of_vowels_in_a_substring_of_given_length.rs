use std::collections::VecDeque;

/// Given a string `s` and an integer `k`, return the maximum number of vowel
/// letters in any substring of `s` with length `k`.
struct Solution;

impl Solution {

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut current = 0;
        let mut max = 0;

        let mut vowels = VecDeque::new();
        let mut index = 0;
        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowels.push_back(index);
                    current += 1;
                    max = max.max(current);
                }
                _ => { }
            }
            index += 1;
            if index >= k {
                if !vowels.is_empty() {
                    let first = vowels[0];
                    if first <= index - k {
                        vowels.pop_front();
                        current -= 1;
                    }
                }
            }
        }

        max
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abciiidef".to_string();
        let k = 3;
        let result = Solution::max_vowels(s, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = "aeiou".to_string();
        let k = 2;
        let result = Solution::max_vowels(s, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let s = "leetcode".to_string();
        let k = 3;
        let result = Solution::max_vowels(s, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn no_vowels() {
        let s = "bcdfghjklmn".to_string();
        let k = 5;
        let result = Solution::max_vowels(s, k);
        assert_eq!(result, 0);
    }

}
