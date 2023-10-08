use std::collections::HashSet;

/// A string `s` is nice if, for every letter of the alphabet that `s`
/// contains, it appears both in uppercase and lowercase. For example,
/// `"abABB"` is nice because `'A'` and `'a'` appear, and `'B'` and `'b'`
/// appear. However, `"abA"` is not because `'b'` appears, but `'B'` does not.
///
/// Given a string `s`, return the longest substring of `s` that is
/// nice. If there are multiple, return the substring of the earliest
/// occurrence. If there are none, return an empty string.
struct Solution;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    start: usize,
    end: usize,
}

impl State {

    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

}

impl Solution {

    fn bad_letters(letters: &Vec<char>, state: State) -> HashSet<char> {
        let mut result = HashSet::new();

        let mut seen = HashSet::new();
        for i in state.start..state.end {
            seen.insert(letters[i]);
        }

        for letter in &seen {
            if letter.is_ascii_uppercase() {
                if !seen.contains(&letter.to_ascii_lowercase()) {
                    result.insert(*letter);
                }
            } else {
                if !seen.contains(&letter.to_ascii_uppercase()) {
                    result.insert(*letter);
                }
            }
        }

        result
    }

    fn worker(letters: &Vec<char>, state: State) -> State {
        let bad = Self::bad_letters(&letters, state);
        let mut result = State::new(0, 0);
        let mut start = state.start;

        for i in state.start..state.end {
            let letter = letters[i];
            if bad.contains(&letter) {
                if start < i {
                    let test_state = State::new(start, i);
                    let answer = Self::worker(letters, test_state);
                    if answer.len() > result.len() {
                        result = answer;
                    }
                }
                start = i + 1;
            }
        }
        if start == state.start {
            result = state;
        } else if start < state.end {
            let test_state = State::new(start, state.end);
            let answer = Self::worker(letters, test_state);
            if answer.len() > result.len() {
                result = answer;
            }
        }

        result
    }

    pub fn longest_nice_substring(s: String) -> String {
        let letters: Vec<char> = s.chars().collect();
        let n = letters.len();
        let initial_state = State::new(0, n);
        let result_state = Self::worker(&letters, initial_state);

        if result_state.len() == 0 {
            "".to_string()
        } else {
            letters[result_state.start..result_state.end].iter().collect()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "YazaAay".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "aAa");
    }

    #[test]
    fn example_2() {
        let s = "Bb".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "Bb");
    }

    #[test]
    fn example_3() {
        let s = "c".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "");
    }

    #[test]
    fn example_4() {
        let s = "jcJ".to_string();
        let result = Solution::longest_nice_substring(s);
        assert_eq!(result, "");
    }

}
