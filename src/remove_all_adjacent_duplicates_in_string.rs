/// You are given a string `s` consisting of lowercase English letters. A
/// duplicate removal consists of choosing two adjacent and equal letters and
/// removing them.
///
/// We repeatedly make duplicate removals on `s` until we no longer can.
///
/// Return the final string after all such duplicate removals have been made.
/// It can be proven that the answer is unique.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn remove_duplicates(_s: String) -> String {
        "".to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let s = "abbaca".to_string();
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, "ca");
    }

    #[ignore]
    #[test]
    fn example_2() {
        let s = "azxxzy".to_string();
        let result = Solution::remove_duplicates(s);
        assert_eq!(result, "ay");
    }

}
