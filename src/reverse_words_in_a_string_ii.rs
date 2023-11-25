/// Given a character array `s`, reverse the order of the words.
///
/// A word is defined as a sequence of non-space characters. The words in `s` will be separated by
/// a single space.
///
/// Your code must solve the problem in-place, i.e. without allocating extra space.
struct Solution;

impl Solution {

    pub fn reverse(s: &mut Vec<char>, start: usize, end: usize) {
        let mut i = start;
        let mut j = end;

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    pub fn reverse_words(s: &mut Vec<char>) {
        let n = s.len();

        // Reverse all letters
        Self::reverse(s, 0, n-1);

        let mut start = 0;
        for i in 0..n {
            if s[i] == ' ' {
                Self::reverse(s, start, i-1);
                start = i + 1;
            }
        }
        Self::reverse(s, start, n-1);
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut s: Vec<char> = "the sky is blue".chars().collect();
        Solution::reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "blue is sky the");
    }

    #[test]
    fn example_2() {
        let mut s = vec!['a'];
        Solution::reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "a");
    }

}
