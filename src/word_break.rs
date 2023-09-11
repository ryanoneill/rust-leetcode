use std::collections::HashMap;

/// Given a string `s` and a dictionary of strings `wordDict`, return `true` if `s` can be
/// segmented into a space-separated sequence of one or more dictionary words.
///
/// Note that the same word in the dictionary may be reused multiple times in the segmentation.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<usize, bool>, word_dict: &Vec<String>, s: &str, index: usize) -> bool {
        if results.contains_key(&index) {
            results[&index]
        } else {
            let mut result = false;
            let n = s.len();
            let wd = word_dict.len();
            for i in 0..wd {
                let word = word_dict[i].as_str();
                let word_len = word.len();
                let end = index + word_len;
                if end <= n {
                    if &s[index..end] == word {
                        if end == n {
                            result = true;
                        } else {
                            result = Self::worker(results, word_dict, s, end);
                        }
                        if result {
                            break;
                        }
                    }
                }
            }
            results.insert(index, result);
            result
        }
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut results = HashMap::new();
        Self::worker(&mut results, &word_dict, &s, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let result = Solution::word_break(s, word_dict);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        let result = Solution::word_break(s, word_dict);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let s = "catsandog".to_string();
        let word_dict = vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()];
        let result = Solution::word_break(s, word_dict);
        assert!(!result);
    }

}
