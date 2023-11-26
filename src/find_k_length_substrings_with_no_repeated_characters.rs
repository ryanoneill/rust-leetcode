use std::collections::HashSet;

/// Given a string `s` and an integer `k`, return the number of substrings in `s` of length `k`
/// with no repeated characters.
struct Solution;

impl Solution {

    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();
        let k = k as usize;

        let mut result = 0;

        if k <= n {
            let mut left = 0;
            let mut seen = HashSet::new();

            for i in 0..n {
                let diff = i - left;
                if diff == k {
                    let old = letters[left];
                    seen.remove(&old);
                    left += 1;
                }

                let letter = letters[i];
                if seen.contains(&letter) {
                    loop {
                        let old = letters[left];
                        seen.remove(&old);
                        left += 1;
                        if old == letter {
                            break;
                        }
                    }
                } 
                seen.insert(letter);
                let diff = i - left + 1;
                if diff == k {
                    result += 1;
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
        let s = str!("havefunonleetcode");
        let k = 5;
        let result = Solution::num_k_len_substr_no_repeats(s, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let s = str!("home");
        let k = 5;
        let result = Solution::num_k_len_substr_no_repeats(s, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let s = str!("abcabcabcabcdabc");
        let k = 4;
        let result = Solution::num_k_len_substr_no_repeats(s, k);
        assert_eq!(result, 4);
    }

}
