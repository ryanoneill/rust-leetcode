/// A string `s` is nice if, for every letter of the alphabet that `s`
/// contains, it appears both in uppercase and lowercase. For example,
/// `"abABB"` is nice because `'A'` and `'a'` appear, and `'B'` and `'b'`
/// appear. However, `"abA"` is not because `'b'` appears, but `'B'` does not.
///
/// Given a string `s`, return the longest substring of `s` that is
/// nice. If there are multiple, return the substring of the earliest
/// occurrence. If there are none, return an empty string.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn longest_nice_substring(_s: String) -> String {
        "".to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let s = "YazaAay".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "aAa");
    }

    #[ignore]
    #[test]
    fn example_2() {
        let s = "Bb".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "Bb");
    }

    #[ignore]
    #[test]
    fn example_3() {
        let s = "c".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "");
    }

}
