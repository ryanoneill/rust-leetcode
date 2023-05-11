/// Given an encoded string, return its decoded string.
///
/// The encoding rule is: `k[encoded_string]`, where the `encoded_string`
/// inside the square brackets is being repeated exactly `k` times. Note that
/// `k` is guaranteed to be a positive integer.
///
/// You may assume that the input string is always valid; there are no extra
/// white spaces, square brackets are well-formed, etc. Furthermore, you may
/// assume that the original data does not contain any digits and that digits
/// are only for those repeat numbers, `k`. For example, there will not be
/// input like `3a` or `2[4]`.
///
/// The test cases are generated so that the length of the output will never
/// exceed `10^5`.
struct Solution;

impl Solution {

    pub fn decode_string(s: String) -> String {
        let mut word_stack = Vec::new();
        let mut times_stack = Vec::new();

        let mut result = Vec::new();
        let mut number = Vec::new();

        for c in s.chars() {
            match c {
                '[' => {
                    let s = String::from_iter(result.iter());
                    word_stack.push(s);
                    result.clear();

                    let s = String::from_iter(number.iter());
                    let num: usize = s.parse().unwrap();
                    times_stack.push(num);
                    number.clear();
                }
                ']' => {
                    let times = times_stack.pop().unwrap();
                    let mut word = word_stack.pop().unwrap();
                    let s = String::from_iter(result.iter());
                    let part = s.repeat(times);

                    word += &part;
                    result.clear();
                    result = word.chars().collect();
                }
                _ if c.is_digit(10) => {
                    number.push(c);
                }
                _ => {
                    result.push(c);
                }
            }

        }

        String::from_iter(result.iter())
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "3[a]2[bc]".to_string();
        let result = Solution::decode_string(s);
        assert_eq!(result, "aaabcbc");
    }

    #[test]
    fn example_2() {
        let s = "3[a2[c]]".to_string();
        let result = Solution::decode_string(s);
        assert_eq!(result, "accaccacc");
    }

    #[test]
    fn example_3() {
        let s = "2[abc]3[cd]ef".to_string();
        let result = Solution::decode_string(s);
        assert_eq!(result, "abcabccdcdcdef");
    }

}
