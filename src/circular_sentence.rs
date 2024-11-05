struct Solution;

impl Solution {

    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut result = true;

        let letters: Vec<char> = sentence.chars().collect();
        let n = letters.len();

        for i in 0..n {
            if letters[i] == ' ' {
                // Safe because no leading or trailing spaces
                let before = letters[i-1];
                let after = letters[i+1];
                if before != after {
                    result = false;
                    break;
                }
            }
        }

        if result {
            result = letters[0] == letters[n-1];
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let sentence = str!("leetcode exercises sound delightful");
        let result = Solution::is_circular_sentence(sentence);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let sentence = str!("eetcode");
        let result = Solution::is_circular_sentence(sentence);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let sentence = str!("Leetcode is cool");
        let result = Solution::is_circular_sentence(sentence);
        assert!(!result);
    }

}
