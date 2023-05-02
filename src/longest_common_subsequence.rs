use std::cmp::max;
use std::collections::HashMap;

/// Given two strings `text1` and `text2`, return the length of their longest
/// common subsequence. If there is no common subsequence, return `0`.
///
/// A subsequence of a string is a new string generated from the original
/// string with some characters (can be none) deleted without changing the
/// relative order of the remaining characters.
///
/// * For example, `"ace"` is a subsequence of `"abcde"`.
///
/// A common subsequence of two strings is a subsequence that is common to both
/// strings.
struct Solution;

impl Solution {

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut maxes = HashMap::new();
        let chars1 = text1.chars().collect();
        let chars2 = text2.chars().collect();
        Self::worker(&mut maxes, &chars1, &chars2, 0, 0)
    }

    fn worker(
        maxes: &mut HashMap<(usize, usize), i32>,
        chars1: &Vec<char>,
        chars2: &Vec<char>,
        i: usize,
        j: usize
    ) -> i32 {
        if i == chars1.len() { 0 }
        else if j == chars2.len() { 0 }
        else if maxes.contains_key(&(i, j)) {
            maxes[&(i, j)]
        } else {
            let result;
            let c1 = chars1[i];
            let c2 = chars2[j];
            if c1 == c2 {
                result = 1 + Self::worker(maxes, chars1, chars2, i + 1, j + 1);
            } else {
                let plus_i = Self::worker(maxes, chars1, chars2, i + 1, j);
                let plus_j = Self::worker(maxes, chars1, chars2, i, j + 1);
                result = max(plus_i, plus_j);
            }
            maxes.insert((i, j), result);
            result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let result = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        let result = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        let result = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(result, 0);
    }

}
