use std::collections::HashMap;
use std::iter::FromIterator;

/// Given an array of strings `strs`, group the anagrams together. You can return the answer in any
/// order.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
struct Solution;

impl Solution {

    fn get_key(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        String::from_iter(chars)
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::new();

        for s in strs {
            let key = Self::get_key(&s);
            groups
                .entry(key)
                .or_insert(Vec::new())
                .push(s);
        }

        groups.into_values().collect()
    }

}

#[cfg(test)]
mod tests {
    use crate::vec_additions::VecAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let mut result = Solution::group_anagrams(strs);
        for item in result.iter_mut() {
            item.sort();
        }
        result.sort();
        assert_eq!(result, vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]);
    }

    #[test]
    fn example_2() {
        let strs = vec![""];
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::group_anagrams(strs);
        assert_eq!(result, vec![vec![""]]);
    }

    #[test]
    fn example_3() {
        let strs = vec!["a"];
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::group_anagrams(strs);
        assert_eq!(result, vec![vec!["a"]]);
    }

}
