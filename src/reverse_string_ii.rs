/// Given a string `s` and an integer `k`, reverse the first `k` characters for every `2k`
/// characters counting from the start of the string.
///
/// If there are fewer than `k` characters left, reverse all of them. If there are less than `2k`
/// but greater than or equal to `k` characters, then reverse the first `k` characters and leave
/// the other as original.
struct Solution;

impl Solution {

    pub fn reverse_str(s: String, k: i32) -> String {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        let mut reversed = false;

        let k = k as usize;
        let mut index = 0;

        for c in s.chars() {
            if index % (2 * k) == 0 {
                reversed = true;
            } else if index % k == 0 {
                while !stack.is_empty() {
                    let letter = stack.pop().unwrap();
                    result.push(letter);
                }
                reversed = false;
            }

            if reversed {
                stack.push(c);
            } else {
                result.push(c);
            }

            index += 1;
        }
        while !stack.is_empty() {
            let letter = stack.pop().unwrap();
            result.push(letter);
        }

        result.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("abcdefg");
        let k = 2;
        let result = Solution::reverse_str(s, k);
        assert_eq!(result, "bacdfeg");
    }

    #[test]
    fn example_2() {
        let s = str!("abcd");
        let k = 2;
        let result = Solution::reverse_str(s, k);
        assert_eq!(result, "bacd");
    }

    #[test]
    fn example_3() {
        let s = str!("abc");
        let k = 4;
        let result = Solution::reverse_str(s, k);
        assert_eq!(result, "cba");
    }

}
