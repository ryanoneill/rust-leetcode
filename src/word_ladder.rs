use std::collections::HashSet;
use std::collections::VecDeque;

/// A transformation sequence from word `beginWord` to word `endWord` using a dictionary `wordList`
/// is a sequence of words `beginWord -> s1 -> s2 -> ... -> sk` such that:
///
/// * Every adjacent pair of words differs by a single letter.
///
/// * Every `si` for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in
/// `wordList`.
///
/// * `sk == endWord`
///
/// * Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return the number of
/// words in the shortest transformation sequence from `beginWord` to `endWord`, or `0` if no such
/// sequence exists.
struct Solution;

impl Solution {

    fn distance(word1: &String, word2: &String) -> usize {
        let mut result = 0;

        let mut word1_chars = word1.chars();
        let mut word2_chars = word2.chars();

        loop {
            match (word1_chars.next(), word2_chars.next()) {
                (Some(a), Some(b)) => {
                    if a != b {
                        result += 1;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        result
    }

    // TODO: Preprocess word_list to make the solution faster
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut finished = false;
        let mut result = 0;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        seen.insert(begin_word.clone());
        queue.push_back(begin_word);
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let word = queue.pop_front().unwrap();
                if word == end_word {
                    finished = true;
                    break;
                } else {
                    for possible in &word_list {
                        if !seen.contains(possible) && Self::distance(&word, possible) == 1 {
                            seen.insert(possible.clone());
                            queue.push_back(possible.clone());
                        }
                    }
                }
            }
            result += 1;
            if finished {
                break;
            } 
        }

        if finished { result } else { 0 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn vec_strings(items: Vec<&str>) -> Vec<String> {
        items
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec_strings(vec!["hot","dot","dog","lot","log","cog"]);
        let result = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec_strings(vec!["hot","dot","dog","lot","log"]);
        let result = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(result, 0);
    }

}
