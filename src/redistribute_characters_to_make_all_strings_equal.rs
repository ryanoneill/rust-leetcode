/// You are given an array of strings `words` (0-indexed).
///
/// In one operation, pick two distinct indices `i` and `j`, where `words[i]` is a non-empty
/// string, and move any character from `words[i]` to any position in `words[j]`.
///
/// Return `true` if you can make every string in `words` equal using any number of operations, and
/// `false` otherwise.
struct Solution;

impl Solution {

    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = [0; 26];
        let mut result = true;

        let a_index = 'a' as usize;
        let n = words.len();

        for word in words {
            for letter in word.chars() {
                let index = letter as usize - a_index;
                counts[index] += 1;
            }
        }

        for i in 0..26 {
            if counts[i] % n != 0 {
                result = false;
                break;
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
        let words = vec!["abc".to_string(), "aabc".to_string(), "bc".to_string()];
        let result = Solution::make_equal(words);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let words = vec!["ab".to_string(), "a".to_string()];
        let result = Solution::make_equal(words);
        assert!(!result);
    }

}
