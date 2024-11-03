/// A fancy string is a string where no three consecutive characters are equal.
///
/// Given a string `s`, delete the minimum possible number of characters from `s` to make it fancy.
///
/// Return the final string after the deletion. It can be shown that the answer will always be
/// unique.
struct Solution;

impl Solution {

    pub fn make_fancy_string(s: String) -> String {
        let mut result: Vec<char> = Vec::new();

        let mut last = ' ';
        let mut count = 0;

        for c in s.chars() {
            if last == c {
                count += 1;
            } else {
                last = c;
                count = 1;
            }
            if count < 3 {
                result.push(c);
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
        let s = str!("leeetcode");
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, "leetcode");

    }

    #[test]
    fn example_2() {
        let s = str!("aaabaaaa");
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, "aabaa");
    }

    #[test]
    fn example_3() {
        let s = str!("aab");
        let result = Solution::make_fancy_string(s);
        assert_eq!(result, "aab");
    }


}
