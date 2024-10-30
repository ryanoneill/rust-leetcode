/// You are given two strings `s` and `t` of the same length and an integer `maxCost`.
///
/// You want to change `s` to `t`. Changing the `ith` character of `s` to `ith` character of `t`
/// costs `|s[i] - t[i]|` (i.e., the absolute difference between the ASCII values of the
/// characters).
///
/// Return the maximum length of a substring `s` that can be changed to be the same as the
/// corresponding substring of `t` with a cost less than or equal to `maxCost`. If there is no
/// substring from `s` that can be changed to its corresponding substring from `t`, return `0`.
struct Solution;

impl Solution {

    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let n = s.len();
        let mut diffs: Vec<i32> = vec![0; n];

        let s_letters: Vec<char> = s.chars().collect();
        let t_letters: Vec<char> = t.chars().collect();

        for index in 0..n {
            let s_char = s_letters[index];
            let t_char = t_letters[index];
            let diff = (s_char as i32 - t_char as i32).abs();
            diffs[index] = diff;
        }

        let mut max_cost = max_cost;
        let mut result = 0;
        let mut current = 0;
        let mut left = 0;

        for right in 0..n {
            max_cost -= diffs[right];
            current += 1;
            if max_cost >= 0 {
                result = result.max(current);
            } else {
                while max_cost < 0 && left <= right {
                    max_cost += diffs[left];
                    current -= 1;
                    left += 1;
                }
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
        let s = str!("abcd");
        let t = str!("bcdf");
        let max_cost = 3;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = str!("abcd");
        let t = str!("cdef");
        let max_cost = 3;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let s = str!("abcd");
        let t = str!("acde");
        let max_cost = 0;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let s = str!("krrgw");
        let t = str!("zjxss");
        let max_cost = 19;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_5() {
        let s = str!("anryddgaqpjdw");
        let t = str!("zjhotgdlmadcf");
        let max_cost = 5;
        let result = Solution::equal_substring(s, t, max_cost);
        assert_eq!(result, 1);
    }

}
