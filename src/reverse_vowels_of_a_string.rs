/// Given a string `s`, reverse only all the vowels in the string and return
/// it. 
///
/// The vowels are `'a'`, `'e'`, `'i'`, `'o'`, `'u'`, and they can appear in
/// both lower and upper cases, more than once.
struct Solution;

impl Solution {

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        }
    }

    pub fn reverse_vowels(s: String) -> String {
        let n = s.len();
        let mut letters: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = n-1;
        
        while left < right {
            while left < n && !Self::is_vowel(letters[left]) {
                left += 1;
            }
            while right > 0 && !Self::is_vowel(letters[right]) {
                right -= 1;
            }
            if left < right {
                letters.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        letters.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "hello".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "holle");
    }

    #[test]
    fn example_2() {
        let s = "leetcode".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "leotcede");
    }

}
