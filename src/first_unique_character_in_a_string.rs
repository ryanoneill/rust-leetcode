use std::collections::HashMap;
use std::collections::HashSet;

/// Given a string `s`, find the first non-repeating character in it and return
/// its index. If it does not exist, return `-1`.
struct Solution;

impl Solution {

    fn to_counts(s: &str) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        for c in s.chars() {
            result.entry(c)
                .and_modify(|count| { *count += 1; })
                .or_insert(1);
        }
        result
    }

    pub fn first_uniq_char(s: String) -> i32 {
        let counts = Self::to_counts(&s);

        let mut result = -1;
        for (i, c) in s.chars().enumerate() {
            if counts[&c] == 1 {
                result = i as i32;
                break;
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
        let s = "leetcode".to_string();
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let s = "loveleetcode".to_string();
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let s = "aabb".to_string();
        let result = Solution::first_uniq_char(s);
        assert_eq!(result, -1);
    }

}
