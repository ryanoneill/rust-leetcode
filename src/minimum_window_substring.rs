use std::collections::HashMap;

/// Given two strings `s` and `t` of lengths `m` and `n` respectively, return the minimum window
/// substring of `s` such that every character in `t` (including duplicates) is included in the
/// window. If there is no such substring, return the empty string `""`.
///
/// The testcases will be generated such that the answer is unique.
///
struct Solution;

impl Solution {

    fn counts(t: String) -> HashMap::<char, usize> {
        let mut result = HashMap::new();
        for letter in t.chars() {
            result
                .entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        result
    }

    fn contains(s_counts: &HashMap<char, usize>, t_counts: &HashMap<char, usize>) -> bool {
        let mut result = true;

        for (key, value) in t_counts {
            result = s_counts.get(key).copied().unwrap_or(0) >= *value;
            if !result {
                break;
            }
        }

        result
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut result = "".to_string();

        let s_len = s.len();
        let t_len = t.len();

        if t_len > s_len || s_len == 0 || t_len == 0 {
            result
        } else {
            let s_letters: Vec<char> = s.chars().collect();
            let t_counts: HashMap<char, usize> = Self::counts(t);
            let mut s_counts: HashMap<char, usize> = HashMap::new();

            for i in 0..t_len {
                let letter = s_letters[i];
                s_counts
                    .entry(letter)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            let mut left = 0;
            let mut right = t_len - 1;
            loop {
                if Self::contains(&s_counts, &t_counts) {
                    let length = right - left + 1;
                    if result.len() == 0 || result.len() > length {
                        result = s_letters[left..=right].iter().collect();
                    }
                    loop {
                        if left == s_len - 1 {
                            break;
                        } else {
                            let letter = s_letters[left];
                            s_counts
                                .entry(letter)
                                .and_modify(|count| *count -= 1);
                            left += 1;

                            if Self::contains(&s_counts, &t_counts) {
                                let length = right - left + 1;
                                if result.len() > length {
                                    result = s_letters[left..=right].iter().collect();
                                }
                            } else {
                                s_counts
                                    .entry(letter)
                                    .and_modify(|count| *count += 1);
                                left -= 1;
                                break;
                            }
                        }
                    }
                } 
                if right == s_len - 1 {
                    break;
                } else {
                    right += 1;
                    let letter = s_letters[right];
                    s_counts
                        .entry(letter)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "BANC");
    }

    #[test]
    fn example_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "a");
    }

    #[test]
    fn example_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "");
    }

    #[test]
    fn example_4() {
        let s = "cabwefgewcwaefgcf".to_string();
        let t = "cae".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "cwae");
    }

}
