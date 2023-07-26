/// Given a string `s`, return the number of palindromic substrings in it.
///
/// A string is a palindrome when it reads the same backward as forward.
///
/// A substring is a contiguous sequence of characters within the string.
struct Solution;

impl Solution {

    fn worker(s_chars: &Vec<char>, left: usize, right: usize) -> i32 {
        let mut result = 0;

        let n = s_chars.len();
        let mut left = left;
        let mut right = right;
        let mut left_c = s_chars[left];
        let mut right_c = s_chars[right];

        while left_c == right_c {
            result += 1;
            if left == 0 || right == n-1 {
                break;
            } else {
                left -= 1;
                right += 1;
                left_c = s_chars[left];
                right_c = s_chars[right];
            }
        }
        result
    }

    pub fn count_substrings(s: String) -> i32 {
        let mut result = 0;

        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();

        for i in 0..n {
            result += Self::worker(&s_chars, i, i);
        }

        for i in 0..n-1 {
            result += Self::worker(&s_chars, i, i+1);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "abc".to_string();
        let result = Solution::count_substrings(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = "aaa".to_string();
        let result = Solution::count_substrings(s);
        assert_eq!(result, 6);
    }

}
