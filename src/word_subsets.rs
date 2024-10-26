use crate::lowercase_letter_counter::LowercaseLetterCounter;

/// You are given two string arrays `words1` and `words2`.
///
/// A string `b` is a subset of string `a` if every letter in `b` occurs in `a` including
/// multiplicity.
///
/// * For example, `"wrr"`, is a subset of `"warrior"` but is not a subset of `"world"`.
///
/// A string `a` from `words1` is universal if for every string `b` in `words2`, `b` is a subset of
/// `a`.
///
/// Return an array of all the universal strings in `words1`. You may return the answer in any
/// order.
struct Solution;

impl Solution {

    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut counts = LowercaseLetterCounter::new("");
        for word in words2 {
            counts.max_in_place(&word);
        }

        let mut results = vec![];
        for word in words1 {
            if counts.is_subset(&word) {
                results.push(word);
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words1 = vec!["amazon", "apple", "facebook", "google", "leetcode"];
        let words1 = words1.iter().map(|w| { w.to_string() }).collect();
        let words2 = vec!["e", "o"];
        let words2 = words2.iter().map(|w| { w.to_string() }).collect();
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, vec!["facebook", "google", "leetcode"]);
    }

    #[test]
    fn example_2() {
        let words1 = vec!["amazon", "apple", "facebook", "google", "leetcode"];
        let words1 = words1.iter().map(|w| { w.to_string() }).collect();
        let words2 = vec!["l", "e"];
        let words2 = words2.iter().map(|w| { w.to_string() }).collect();
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, vec!["apple", "google", "leetcode"]);
    }

}
