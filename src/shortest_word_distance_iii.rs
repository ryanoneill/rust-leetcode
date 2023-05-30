/// Given an array of strings `wordsDict` and two strings that already exist in the array `word1`
/// and `word2`, return the shortest distance between the occurrence of these two words in the
/// list.
///
/// Note that `word1` and `word2` may be the same. It is guaranteed that they represent two
/// individual words in the list.
struct Solution;

impl Solution {

    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let same_word = word1 == word2;
        let mut last_word1_index = i32::MAX;
        let mut last_word2_index = i32::MAX;

        let mut result = i32::MAX;

        let n = words_dict.len();
        for i in 0..n {
            let word = &words_dict[i];
            let i = i as i32;
            if same_word && word == &word1 {
                if last_word1_index == i32::MAX {
                    last_word1_index = i;
                } else if last_word2_index == i32::MAX {
                    last_word2_index = i;
                } else if last_word1_index < last_word2_index {
                    last_word1_index = i;
                } else {
                    last_word2_index = i;
                }

                let distance = (last_word2_index - last_word1_index).abs();
                result = result.min(distance);
            } else if word == &word1 {
                last_word1_index = i;

                let distance = (last_word2_index - last_word1_index).abs();
                result = result.min(distance);
            } else if word == &word2 {
                last_word2_index = i;

                let distance = (last_word2_index - last_word1_index).abs();
                result = result.min(distance);
            }
        }

        result

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words_dict = vec!["practice", "makes", "perfect", "coding", "makes"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = Solution::shortest_word_distance(words_dict, "makes".to_string(), "coding".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let words_dict = vec!["practice", "makes", "perfect", "coding", "makes"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = Solution::shortest_word_distance(words_dict, "makes".to_string(), "makes".to_string());
        assert_eq!(result, 3);
    }

}
