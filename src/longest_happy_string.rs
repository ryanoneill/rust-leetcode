use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    count: i32,
    letter: char,
}

impl State {

    pub fn new(count: i32, letter: char) -> Self {
        Self {
            count,
            letter
        }
    }

}

/// A string `s` is called happy if it satisfies the following conditions:
///
/// * `s` only contains the letters `'a'`, `'b'`, and `'c'`.
///
/// * `s` does not contain any of `"aaa"`, `"bbb"`, or `"ccc"` as a substring.
///
/// * `s` contains at most `a` occurrences of the letter `'a'`.
///
/// * `s` contains at most `b` occurrences of the letter `'b'`.
///
/// * `s` contains at most `c` occurrences of the letter `'c'`.
///
/// Given three integers `a`, `b`, and `c`, return the longest possible happy string. If there are
/// multiple longest happy strings, return any of them. If there is no such string, return the
/// empty string `""`.
///
/// A substring is a contiguous sequence of characters within a string.
struct Solution;

impl Solution {

    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        if a > 0 {
            heap.push(State::new(a, 'a'));
        }
        if b > 0 {
            heap.push(State::new(b, 'b'));
        }
        if c > 0 {
            heap.push(State::new(c, 'c'));
        }

        let mut letters: Vec<char> = vec![];
        let mut last = ' ';
        let mut last_count = 0;

        while !heap.is_empty() {
            let mut state = heap.pop().unwrap();
            if last == state.letter {
                if last_count == 2 {
                    if heap.is_empty() {
                        break;
                    } else {
                        let mut alt = heap.pop().unwrap();
                        letters.push(alt.letter);
                        alt.count -= 1;
                        if alt.count > 0 {
                            heap.push(alt);
                        }
                        last = alt.letter;
                    }
                } else {
                    letters.push(state.letter);
                    state.count -= 1;
                    last_count += 1;
                }
                if state.count > 0 {
                    heap.push(state);
                }
            } else {
                letters.push(state.letter);
                state.count -= 1;
                if state.count > 0 {
                    heap.push(state);
                }
                last = state.letter;
                last_count = 1;
            }
        }
        
        let result: String = letters.into_iter().collect();
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let a = 1;
        let b = 1;
        let c = 7;
        let result = Solution::longest_diverse_string(a, b, c);
        assert_eq!(result, "ccbccacc");
    }

    #[test]
    fn example_2() {
        let a = 7;
        let b = 1;
        let c = 0;
        let result = Solution::longest_diverse_string(a, b, c);
        assert_eq!(result, "aabaa");
    }

    #[test]
    fn example_3() {
        let a = 0;
        let b = 0;
        let c = 0;
        let result = Solution::longest_diverse_string(a, b, c);
        assert_eq!(result, "");
    }

    #[test]
    fn example_4() {
        let a = 0;
        let b = 1;
        let c = 0;
        let result = Solution::longest_diverse_string(a, b, c);
        assert_eq!(result, "b");
    }

}
