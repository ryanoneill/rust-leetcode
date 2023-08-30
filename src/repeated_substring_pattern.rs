// Given a string `s`, check if it can be constructed by taking a substring of it and appending
// multiple copies of the substring together.
struct Solution;

impl Solution {

    pub fn repeated_substring_pattern(s: String) -> bool {
        let doubled = s.clone() + &s;
        let sub = &doubled[1..doubled.len()-1];
        sub.contains(&s)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abab".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = "aba".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s = "abcabcabcabc".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert!(result);
    }

}
