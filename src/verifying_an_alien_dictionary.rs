/// In an alien language, surprisingly, they also use English lowercase letters, but possibly in a
/// different `order`. The `order` of the alphabet is some permutation of lowercase letters.
///
/// Given a sequence of `words` written in the alien language, and the `order` of the alphabet,
/// return `true` if and only if the given `words` are sorted lexicographically in this alien
/// language.
struct Solution;

impl Solution {

    fn fill_in_sequence(sequence: &mut Vec<usize>, order: String) {
        let mut value = 0;
        for letter in order.chars() {
            let index = letter as usize - 'a' as usize;
            sequence[index] = value;
            value += 1;
        }
    }

    fn are_sorted(word1: &str, word2: &str, sequence: &Vec<usize>) -> bool {
        let result;

        let mut word1_chars = word1.chars();
        let mut word2_chars = word2.chars();

        loop {
            match (word1_chars.next(), word2_chars.next()) {
                (Some(l1), Some(l2)) => {
                    let l1_index = l1 as usize - 'a' as usize;
                    let l2_index = l2 as usize - 'a' as usize;

                    let l1_value = sequence[l1_index];
                    let l2_value = sequence[l2_index];

                    if l1_value == l2_value {
                        // continue
                    } else if l1_value < l2_value {
                        result = true;
                        break;
                    } else {
                        result = false;
                        break;
                    }
                }
                (None, Some(_)) => {
                    result = true;
                    break;
                }
                (Some(_), None) => {
                    result = false;
                    break;
                }
                (_, _) => {
                    result = true;
                    break;
                }
            }
        }

        result
    }

    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut sequence = vec![0; 26];
        Self::fill_in_sequence(&mut sequence, order);

        let mut result = true;
        for pair in words.windows(2) {
            result = result && Self::are_sorted(&pair[0], &pair[1], &sequence);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let words = vec!["apple".to_string(), "app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = Solution::is_alien_sorted(words, order);
        assert!(!result);
    }

}
