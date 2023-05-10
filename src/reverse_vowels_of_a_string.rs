use std::collections::VecDeque;

/// Given a string `s`, reverse only all the vowels in the string and return
/// it. 
///
/// The vowels are `'a'`, `'e'`, `'i'`, `'o'`, `'u'`, and they can appear in
/// both lower and upper cases, more than once.
struct Solution;

impl Solution {

    pub fn reverse_vowels(s: String) -> String {
        let mut stack = Vec::new();
        let mut queue = VecDeque::new();

        let n = s.len();
        let mut letters: Vec<char> = s.chars().collect();

        for i in 0..n {
            let letter = letters[i];
            match letter {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    queue.push_back(i);
                    stack.push(letter);
                }
                _ => { }
            }
        }

        let q = queue.len();
        for _ in 0..q {
            let index = queue.pop_front().unwrap();
            let letter = stack.pop().unwrap();
            letters[index] = letter;
        }

        letters.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "hello".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "holle");
    }

    #[test]
    fn example_2() {
        let s = "leetcode".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "leotcede");
    }

}
