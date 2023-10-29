use std::collections::{HashMap, HashSet};

/// Two strings are considered close if you can attain one from the other using
/// the following operations:
///
/// * Operation 1: Swap any two existing characters.
///   * For example, `abcde -> aecdb`
///
/// * Operation 2: Transform every occurrence of one existing character into
///   another existing character, and do the same with the other character.
///   * For example, `aacabb -> bbcbaa` (all `a`'s turn into `b`'s, and all
///     `b`'s turn into `a`'s)
///
/// You can use the operations on either string as many times as necessary.
///
/// Given two strings, `word1` and `word2`, return `true` if `word1` and
/// `word2` are close, and `false` otherwise.
struct Solution;

impl Solution {

    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut freq1 = HashMap::new();
        for c in word1.chars() {
            freq1.entry(c)
                .and_modify(|count| { *count += 1; })
                .or_insert(1);
        }

        let mut freq2 = HashMap::new();
        for c in word2.chars() {
            freq2.entry(c)
                .and_modify(|count| { *count += 1; })
                .or_insert(1);
        }

        let keys1: HashSet<char> = HashSet::from_iter(freq1.keys().copied());
        let keys2: HashSet<char> = HashSet::from_iter(freq2.keys().copied());

        let mut result = keys1 == keys2;
        if result {
            let mut counts1: Vec<i32> = freq1.into_values().collect();
            counts1.sort();

            let mut counts2: Vec<i32> = freq2.into_values().collect();
            counts2.sort();
            result = counts1 == counts2
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let word1 = str!("abc");
        let word2 = str!("bca");
        let result = Solution::close_strings(word1, word2);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let word1 = str!("a");
        let word2 = str!("aa");
        let result = Solution::close_strings(word1, word2);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let word1 = str!("cabbba");
        let word2 = str!("abbccc");
        let result = Solution::close_strings(word1, word2);
        assert!(result);
    }

    #[test]
    fn real_world_1() {
        let word1 = str!("uau");
        let word2 = str!("ssx");
        let result = Solution::close_strings(word1, word2);
        assert!(!result);
    }

}
