/// Write a function to find the longest common prefix string amongst an array
/// of strings.
///
/// If there is no common prefix, return an empty string `""`.
struct Solution;

impl Solution {

    fn prefix_compare(s1: &str, s2: &str) -> usize {
        let mut result = 0;

        let mut s1_chars = s1.chars();
        let mut s2_chars = s2.chars();

        loop {
            match (s1_chars.next(), s2_chars.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    result += 1;
                }
                _ => { break; }
            }
        }

        result
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs;
        let n = strs.len();

        let mut result = "".to_string();

        if n == 0 { }
        else if n == 1 { result = strs.pop().unwrap(); }
        else {
            let last = strs.pop().unwrap();
            let length = strs.iter()
                .map(|s| { Self::prefix_compare(&last, s) })
                .min()
                .unwrap_or_default();
            if length > 0 {
                result = last.chars().take(length).collect();
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
        let orig = vec!["flower", "flow", "flight"];
        let strs = orig.iter().map(|&s| s.to_string()).collect();
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(result, "fl");
    }

    #[test]
    fn example_2() {
        let orig = vec!["dog", "racecar", "car"];
        let strs = orig.iter().map(|&s| s.to_string()).collect();
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(result, "");
    }

}
