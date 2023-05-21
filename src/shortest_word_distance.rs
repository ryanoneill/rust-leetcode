/// Given an array of strings `wordsDict` and two different strings that
/// already exist in the array `word` and `word2`, return the shortest distance
/// between these two words in the list.
struct Solution;

impl Solution {

    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let mut last_word1_index = i32::MAX;
        let mut last_word2_index = i32::MAX;

        let mut result = i32::MAX;

        let n = words_dict.len();
        for i in 0..n {
            let word = &words_dict[i];
            if word == &word1 {
                last_word1_index = i as i32;

                let distance = (last_word2_index - last_word1_index).abs();
                result = result.min(distance);
            } else if word == &word2 {
                last_word2_index = i as i32;

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
        let orig = vec!["practice", "makes", "perfect", "coding", "makes"];
        let words_dict = orig.iter().map(|s| s.to_string()).collect();
        let word1 = "coding".to_string();
        let word2 = "practice".to_string();
        let result = Solution::shortest_distance(words_dict, word1, word2);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let orig = vec!["practice", "makes", "perfect", "coding", "makes"];
        let words_dict = orig.iter().map(|s| s.to_string()).collect();
        let word1 = "makes".to_string();
        let word2 = "coding".to_string();
        let result = Solution::shortest_distance(words_dict, word1, word2);
        assert_eq!(result, 1);
    }

}
