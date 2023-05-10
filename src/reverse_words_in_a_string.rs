/// Given an input string `s`, reverse the order of the words.
///
/// A word is defined as a sequence of non-space characters. The words in
/// `s` will be separated by at least once space.
///
/// Return a string of the words in reverse order concatenated by a
/// single space.
///
/// Note that `s` may contain leading or tailing spaces or multiple
/// spaces between two words. The returned string should only have a single
/// space separating the words. Do not include any extra spaces.
struct Solution;

impl Solution {

    pub fn reverse_words(s: String) -> String {
        let words: Vec<&str> = s.split(' ')
            .rev()
            .filter(|s| !s.is_empty())
            .collect();
        words.join(" ")
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "the sky is blue".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "blue is sky the");
    }

    #[test]
    fn example_2() {
        let s = "  hello world  ".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "world hello");
    }

    #[test]
    fn example_3() {
        let s = "a good   example".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "example good a");
    }

}
