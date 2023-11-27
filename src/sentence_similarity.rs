use std::collections::HashMap;
use std::collections::HashSet;

/// We can represent a sentence as an array of words, for example, the sentence `"I am happy with
/// leetcode"` can be represented as `arr = ["I", "am", "happy", "with", "leetcode"]`.
///
/// Given two sentences `sentence1` and `sentence2` each represented as a string array and given an
/// array of string pairs `similarPairs` where `similarPairs[i] = [xi, yi]` indicates that the two
/// words `xi` and `yi` are similar.
///
/// Return `true` if `sentence1` and `sentence2` are similar, or `false` if they are not similar.
///
/// Two sentences are similar if:
///
/// * They have the same length (i.e., the same number of words)
/// * `sentence1[i]` and `sentence2[i]` are similar.
///
/// Notice that a word is always similar to itself, also notice that the similarity relation is not
/// transitive. For example, if the words `a` and `b` are similar, and the words `b` and `c` are
/// similar, `a` and `c` are not necessarily similar.
struct Solution;

impl Solution {

    pub fn are_sentences_similar(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
        let mut result = true;
        let m = sentence1.len();
        let n = sentence2.len();

        if m != n {
            result = false;
        } else {
            let mut pairings: HashMap<String, HashSet<String>> = HashMap::new();
            for pair in similar_pairs {
                let word1 = pair[0].clone();
                let word2 = pair[1].clone();
                pairings
                    .entry(word1.clone())
                    .or_insert(HashSet::new())
                    .insert(word2.clone());
                pairings
                    .entry(word2)
                    .or_insert(HashSet::new())
                    .insert(word1);
            }

            for i in 0..n {
                let word1 = &sentence1[i];
                let word2 = &sentence2[i];
                if word1 == word2 {
                    // continue
                } else if pairings.contains_key(word1) && pairings[word1].contains(word2) {
                    // continue
                } else if pairings.contains_key(word2) && pairings[word2].contains(word1) {
                    // continue
                } else {
                    result = false;
                    break;
                }
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
        let sentence1 = vec![str!("great"), str!("acting"), str!("skills")];
        let sentence2 = vec![str!("fine"), str!("drama"), str!("talent")];
        let similar_pairs = vec![
            vec![str!("great"), str!("fine")],
            vec![str!("drama"), str!("acting")],
            vec![str!("skills"), str!("talent")],
        ];
        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let sentence1 = vec![str!("great")];
        let sentence2 = vec![str!("great")];
        let similar_pairs = vec![];
        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let sentence1 = vec![str!("great")];
        let sentence2 = vec![str!("doubleplus"), str!("good")];
        let similar_pairs = vec![
            vec![str!("great"), str!("doubleplus")],
        ];
        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert!(!result);
    }

    #[test]
    fn example_4() {
        let sentence1 = vec![str!("great"), str!("acting"), str!("skills")];
        let sentence2 = vec![str!("fine"), str!("drama"), str!("talent")];
        let similar_pairs = vec![
            vec![str!("great"), str!("fine")],
            vec![str!("drama"), str!("acting")],
            vec![str!("talent"), str!("skills")],
        ];
        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert!(result);
    }

    #[test]
    fn example_5() {
        let sentence1 = vec![str!("great"), str!("acting"), str!("else")];
        let sentence2 = vec![str!("fine"), str!("drama"), str!("talent")];
        let similar_pairs = vec![
            vec![str!("great"), str!("fine")],
            vec![str!("drama"), str!("acting")],
            vec![str!("talent"), str!("something")],
            vec![str!("talent"), str!("else")],
        ];
        let result = Solution::are_sentences_similar(sentence1, sentence2, similar_pairs);
        assert!(result);
    }

}
