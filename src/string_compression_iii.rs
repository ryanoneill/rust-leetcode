/// Given a string `word`, compress it using the following algorithm:
///
/// * Begin with an empty string `comp`. While `word` is not empty, use the following operation:
///
///   * Remove a maximum length prefix of `word` made of a single character `c` repeating at most 9
///     times.
///
///   * Append the length of the prefix followed by `c` to `comp`.
///
/// Return the string `comp`.
struct Solution;

impl Solution {

    pub fn compressed_string(word: String) -> String {
        let n = word.len();

        let mut last = ' ';
        let mut count = 0;

        let mut result = Vec::new();
        for c in word.chars() {
            if last == ' ' {
                last = c;
                count = 1;
            } else if last != c {
                result.push(char::from_digit(count, 10).unwrap());
                result.push(last);

                last = c;
                count = 1;
            } else if count == 9 {
                result.push(char::from_digit(count, 10).unwrap());
                result.push(last);

                count = 1;
            } else {
                count += 1;
            }
        }
        if n > 0 {
            result.push(char::from_digit(count, 10).unwrap());
            result.push(last);
        }

        result.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let word = str!("abcde");
        let result = Solution::compressed_string(word);
        assert_eq!(result, "1a1b1c1d1e");
    }

    #[test]
    fn example_2() {
        let word = str!("aaaaaaaaaaaaaabb");
        let result = Solution::compressed_string(word);
        assert_eq!(result, "9a5a2b");
    }

}
