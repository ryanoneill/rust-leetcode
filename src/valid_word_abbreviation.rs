/// A string can be abbreviated by replacing any number of non-adjacent, non-empty substrings with
/// their lengths. The lengths should not have leading zeros.
///
/// For example, a string such as `"substitution"` could be abbreviated as (but not limited to):
///
/// * "s10n" ("s ubstitutio n")
/// * "sub4u4" ("sub stit u tion")
/// * "12" ("substitution")
/// * "su3i1u2on" ("su bst i t u ti on")
/// * "substitution" (no substrings replaced)
///
/// The following are not valid abbreviations:
///
/// * "s55n" ("s ubsti tutio n", the replaced substrings are adjacent)
/// * "s010n" (has leading zeros)
/// * "s0ubstitution" (replaces an empty substring)
///
/// Given a string `word` and an abbreviation `abbr`, return whether the string
/// matches the given abbreviation.
///
/// A substring is a contiguous non-empty sequence of characters within a string.
struct Solution;

#[derive(PartialEq, Eq)]
enum State {
    Letter,
    Number,
}

impl Solution {

    fn digit_to_number(letter: char) -> usize {
        match letter {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _   => 0,
        }
    }

    // 1 <= word.length <= 20
    // `word` consists of only lowercase English letters.
    // 1 <= abbr.length <= 10
    // `abbr` consists of lowercase English letters and digits.
    // All the integers in `abbr` will fit in a 32-bit integer.
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let letters: Vec<char> = word.chars().collect();
        let n = letters.len();
        let mut index = 0;

        let aletters: Vec<char> = abbr.chars().collect();

        let mut state = State::Letter;
        let mut count = 0;

        let mut result = true;

        for i in 0..aletters.len() {
            let a = aletters[i];
            if a.is_digit(10) {
                state = State::Number;
                let number = Self::digit_to_number(a);
                if number == 0 && count == 0 {
                    result = false;
                    break;
                } else {
                    count *= 10;
                    count += number;
                }
            } else {
                if state == State::Number {
                    index += count;
                    count = 0;
                    state = State::Letter;
                }
                if index < n {
                    let w = letters[index];
                    index += 1;
                    if w != a {
                        result = false;
                        break;
                    } 
                } else {
                    result = false;
                    break;
                }
            }
        }

        if result {
            if state == State::Number {
                index += count;
            }
            if index != n {
                result = false;
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
        let word = "internationalization".to_string();
        let abbr = "i12iz4n".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let word = "apple".to_string();
        let abbr = "a2e".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let word = "substitution".to_string();
        let abbr = "s10n".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_4() {
        let word = "substitution".to_string();
        let abbr = "sub4u4".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_5() {
        let word = "substitution".to_string();
        let abbr = "12".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_6() {
        let word = "substitution".to_string();
        let abbr = "su3i1u2on".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_7() {
        let word = "substitution".to_string();
        let abbr = word.clone();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(result);
    }

    #[test]
    fn example_8() {
        let word = "substitution".to_string();
        let abbr = "s55n".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_9() {
        let word = "substitution".to_string();
        let abbr = "s010n".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_10() {
        let word = "substitution".to_string();
        let abbr = "s0ubstitution".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_11() {
        let word = "substitution".to_string();
        let abbr = "13".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_12() {
        let word = "substitution".to_string();
        let abbr = "s12".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_13() {
        let word = "hello".to_string();
        let abbr = "hello0".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_14() {
        let word = "hello".to_string();
        let abbr = "hell".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

    #[test]
    fn example_15() {
        let word = "hello".to_string();
        let abbr = "hellow".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert!(!result);
    }

}
