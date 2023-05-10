/// Given a string `s`, remove the vowels `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`
/// from it, and return the new string.
struct Solution;

impl Solution {

    pub fn remove_vowels(s: String) -> String {
        let n = s.len();
        let mut result = Vec::with_capacity(n);

        for letter in s.chars() {
            match letter {
                'a' | 'e' | 'i' | 'o' | 'u' => { }
                _ => { result.push(letter); }
            }
        }

        result.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "leetcodeisacommunityforcoders".to_string();
        let result = Solution::remove_vowels(s);
        assert_eq!(result, "ltcdscmmntyfrcdrs");
    }

    #[test]
    fn example_2() {
        let s = "aeiou".to_string();
        let result = Solution::remove_vowels(s);
        assert_eq!(result, "");
    }

    #[test]
    fn no_removals() {
        let s = "bcdfghjkl".to_string();
        let result = Solution::remove_vowels(s);
        assert_eq!(result, "bcdfghjkl");
    }

}
