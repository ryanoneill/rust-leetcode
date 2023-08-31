/// Given an array of keywords `words` and a string `s`, make all appearances of all keywords
/// `words[i]` in `s` bold. Any letters between `<b>` and `</b>` tags become bold.
///
/// Return `s` after adding the bold tags. The returned string should use the least number of tags
/// possible, and the tags should form a valid combination.
struct Solution;

impl Solution {

    pub fn bold_words(words: Vec<String>, s: String) -> String {
        let n = s.chars().count();
        let mut bold = vec![false; n];

        for word in words {
            let m = word.len();
            let mut i = 0;
            while let Some(value) = &s[i..].find(&word) {
                let start = *value + i;
                for j in start..start+m {
                    bold[j] = true;
                }
                i = start + 1;
            }
        }

        let mut result = String::new();

        let mut i = 0;
        for letter in s.chars() {
            if bold[i] {
                if i == 0 || !bold[i-1] {
                    result.push_str("<b>");
                }
            }
            result.push(letter);

            if bold[i] {
                if i == n-1 || !bold[i+1] {
                    result.push_str("</b>");
                }
            }
            i += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let words = vec!["ab".to_string(), "bc".to_string()];
        let s = "aabcd".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "a<b>abc</b>d");
    }

    #[test]
    fn example_2() {
        let words = vec!["ab".to_string(), "cb".to_string()];
        let s = "aabcd".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "a<b>ab</b>cd");
    }

    #[test]
    fn example_3() {
        let words = vec!["ccb".to_string(), "b".to_string(), "d".to_string(), "cba".to_string(), "dc".to_string()];
        let s = "eeaadadadc".to_string();
        let result = Solution::bold_words(words, s);
        assert_eq!(result, "eeaa<b>d</b>a<b>d</b>a<b>dc</b>");
    }

}
