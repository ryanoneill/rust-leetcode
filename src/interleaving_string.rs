use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    s1_index: usize,
    s2_index: usize,
    s3_index: usize
}

impl State {

    pub fn new(s1_index: usize, s2_index: usize, s3_index: usize) -> Self {
        Self { s1_index, s2_index, s3_index }
    }

    fn increment_with_s1(&self) -> Self {
        Self::new(self.s1_index + 1, self.s2_index, self.s3_index + 1)
    }

    fn increment_with_s2(&self) -> Self {
        Self::new(self.s1_index, self.s2_index + 1, self.s3_index + 1)
    }

}

/// Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an interleaving of `s1` and
/// `s2`. An interleaving of two strings `s` and `t` is a configuration where `s` and `t` are
/// divided into `n` and `m` substrings respectively, such that:
///
/// * `s = s1 + s2 + ... + sn`
/// * `t = t1 + t2 + ... + tm`
/// * `|n - m| <= 1`
/// * The interleaving is `s1 + t1 + s2 + t2 + s3 + t3 + ...` or `t1 + s1 + t2 + s2 + t3 + s3 + ...`
///
/// Note: `a + b` is the concatenation of strings `a` and `b`.
struct Solution;

impl Solution {

    fn worker(
        results: &mut HashMap<State, bool>,
        s1_chars: &Vec<char>,
        s2_chars: &Vec<char>,
        s3_chars: &Vec<char>,
        state: State,
    ) -> bool {
        if results.contains_key(&state) {
            results[&state]
        } else {
            let mut result = false;

            let s1n = s1_chars.len();
            let s2n = s2_chars.len();
            let s3n = s3_chars.len();

            if s3n == 0 {
                result = s1n == 0 && s2n == 0;
            } else if state.s3_index == s3n-1 {
                let s3c = s3_chars[state.s3_index];
                if state.s1_index < s1n {
                    let s1c = s1_chars[state.s1_index];
                    result = s1c == s3c && (state.s1_index + 1 == s1n) && (state.s2_index == s2n);
                } else if state.s2_index < s2n {
                    let s2c = s2_chars[state.s2_index];
                    result = s2c == s3c && (state.s2_index + 1 == s2n) && (state.s1_index == s1n);
                }
            } else {
                let s3c = s3_chars[state.s3_index];
                if state.s2_index < s2n {
                    let s2c = s2_chars[state.s2_index];
                    if s2c == s3c {
                        let next_state = state.increment_with_s2();
                        result = result || Self::worker(results, s1_chars, s2_chars, s3_chars, next_state);
                    }
                }
                if state.s1_index < s1n {
                    let s1c = s1_chars[state.s1_index];
                    if s1c == s3c {
                        let next_state = state.increment_with_s1();
                        result = result || Self::worker(results, s1_chars, s2_chars, s3_chars, next_state);
                    }
                }
            }

            results.insert(state, result);

            result
        }
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s3_chars: Vec<char> = s3.chars().collect();
        let initial_state = State::new(0, 0, 0);
        let mut results = HashMap::new();
        Self::worker(&mut results, &s1_chars, &s2_chars, &s3_chars, initial_state)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert!(result);
    }

    #[test]
    fn real_world_1() {
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "a".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert!(!result);
    }

}
