use std::collections::HashSet;
use std::iter::FromIterator;

/// A subsequence of a string is a new string that is formed from the original string by deleting
/// some (can be none) of the characters without disturbing the relative positions of the remaining
/// characters. (i.e. `"ace"` is a subsequence of `"abcde"` while `"aec"` is not).
///
/// Given two strings `source` and `target`, return the minimum number of subsequences of `source`
/// such that their concatenation equals `target`. If the task is impossible, return `-1`.
struct Solution;

impl Solution {

    pub fn shortest_way(source: String, target: String) -> i32 {
        let source_letters: Vec<char> = source.chars().collect();
        let target_letters: Vec<char> = target.chars().collect();

        let source_set: HashSet<char> = HashSet::from_iter(source_letters.iter().copied());
        let target_set: HashSet<char> = HashSet::from_iter(target_letters.iter().copied());

        let mut result = 0;
        for letter in target_set {
            if !source_set.contains(&letter) {
                result = -1;
                break;
            }
        }

        if result == 0 {
            let mut source_index = 0;
            let mut target_index = 0;

            let m = source_letters.len();
            let n = target_letters.len();

            result += 1;
            loop {
                let source_letter = source_letters[source_index];
                let target_letter = target_letters[target_index];

                if source_letter == target_letter {
                    source_index += 1;
                    target_index += 1;
                } else {
                    source_index += 1;
                }

                if target_index == n {
                    break;
                } else if source_index == m {
                    result += 1;
                    source_index = 0;
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
        let source = "abc".to_string();
        let target = "abcbc".to_string();
        let result = Solution::shortest_way(source, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let source = "abc".to_string();
        let target = "acdbc".to_string();
        let result = Solution::shortest_way(source, target);
        assert_eq!(result, -1);
    }

    fn example_3() {
        let source = "xyz".to_string();
        let target = "xzyxz".to_string();
        let result = Solution::shortest_way(source, target);
        assert_eq!(result, 3);
    }

}
