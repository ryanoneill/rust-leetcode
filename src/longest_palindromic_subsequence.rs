use std::collections::HashMap;

/// Given a string `s`, find the longest palindromic subsequence's length in `s`.
///
/// A subsequence is a sequence that can be derived from another sequence by deleting some or no
/// elements without changing the order of the remaining elements.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<(usize, usize), i32>, letters: &Vec<char>, left: usize, right: usize) -> i32 {
        let key = (left, right);

        if results.contains_key(&key) {
            results[&key]
        } else {
            let n = letters.len();
            let mut result = 0;

            if letters[left] == letters[right] {
                if left == right {
                    result += 1;
                } else {
                    result += 2;
                }
                if left != 0 && right != n-1 {
                    result += Self::worker(results, letters, left-1, right+1);
                }
            } else {
                let mut go_left = 0;
                let mut go_right = 0;
                if left != 0 {
                    go_left = Self::worker(results, letters, left-1, right);
                }
                if right != n-1 {
                    go_right = Self::worker(results, letters, left, right+1);
                }
                
                result = go_left.max(go_right);
            }

            results.insert(key, result);
            result
        }
    }

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut result = 0;
        let mut results = HashMap::new();

        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();

        for i in 0..n {
            let centered = Self::worker(&mut results, &letters, i, i);
            result = result.max(centered);
        }

        for i in 0..n-1 {
            let between = Self::worker(&mut results, &letters, i, i+1);
            result = result.max(between);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "bbbab".to_string();
        let result = Solution::longest_palindrome_subseq(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome_subseq(s);
        assert_eq!(result, 2);
    }

}
