/// Given a string `s` consisting of words and spaces, return the length of the
/// last word in the string.
///
/// A word is a maximal substring consisting of non-space characters only.
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace()
            .rev()
            .skip_while(|x| *x == "")
            .next()
            .map(|x| x.len())
            .unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "Hello World".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let s = "   fly me   to   the moon  ".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let s = "luffy is still joyboy".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(result, 6);
    }
}
