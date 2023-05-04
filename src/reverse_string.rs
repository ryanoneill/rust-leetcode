/// Write a function that reverses a string. The input string is given as an
/// array of characters `s`.
///
/// You must do this by modifying the input array in-place with `O(1)` extra
/// memory.
struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let end = n - 1;

        for i in 0..n / 2 {
            s.swap(i, end - i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn example_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
