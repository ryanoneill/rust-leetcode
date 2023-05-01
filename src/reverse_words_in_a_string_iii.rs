/// Given a string `s`, reverse the order of characters in each word within
/// a sentence while still preserving whitespace and initial word order.
struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "Let's take LeetCode contest".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "s'teL ekat edoCteeL tsetnoc");
    }

    #[test]
    fn exmaple_2() {
        let s = "God Ding".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, "doG gniD");
    }
}
