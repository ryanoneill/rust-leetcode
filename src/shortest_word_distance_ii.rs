use std::collections::HashMap;

/// Design a data structure that will be initialized with a string array, and
/// then it should answer queries of the shortest distance between two
/// different strings from the array.
struct WordDistance {
    indices: HashMap<String, Vec<i32>>
}

impl WordDistance {

    fn new(words_dict: Vec<String>) -> Self {
        let mut indices = HashMap::new();
        for (i, word) in words_dict.into_iter().enumerate() {
            indices
                .entry(word)
                .or_insert(Vec::new())
                .push(i as i32);
        }
        Self { indices }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let word1_indices = &self.indices[&word1];
        let word2_indices = &self.indices[&word2];

        let mut word1_i = 0;
        let mut word1_index = word1_indices[word1_i];
        let word1_n = word1_indices.len();
        let mut word2_i = 0;
        let mut word2_index = word2_indices[word2_i];
        let word2_n = word2_indices.len();

        let mut result = (word1_index - word2_index).abs();

        loop {
            if word1_i == word1_n-1 && word2_i == word2_n-1 {
                break;
            } else if word1_i == word1_n-1 {
                word2_i += 1;
            } else if word2_i == word2_n-1 {
                word1_i += 1;
            } else {
                let next1 = word1_indices[word1_i+1];
                let next2 = word2_indices[word2_i+1];
                if next1 < next2 {
                    word1_i += 1;
                } else {
                    word2_i += 1;
                }
            }

            word1_index = word1_indices[word1_i];
            word2_index = word2_indices[word2_i];
            let current = (word1_index - word2_index).abs();
            result = result.min(current);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::WordDistance;

    #[test]
    fn example_1() {
        let words = vec!["practice", "makes", "perfect", "coding", "makes"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let word_distance = WordDistance::new(words);
        let result = word_distance.shortest("coding".to_string(), "practice".to_string());
        assert_eq!(result, 3);
        let result = word_distance.shortest("makes".to_string(), "coding".to_string());
        assert_eq!(result, 1);
    }

}
