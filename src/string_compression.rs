/// Given an array of characters `chars`, compress it using the following
/// algorithm:
///
/// Begin with an empty string `s`. For each group of consecutive repeating
/// characters in `chars`:
///
/// * If the group's length is `1`, append the character to `s`.
///
/// * Otherwise, append the character followed by the group's length.
///
/// The compressed string `s` should not be returned separately, but instead,
/// be stored in the input character array `chars`. Note that group lengths that
/// are `10` or longer will be split into multiple characters in `chars`.
///
/// After you are done modifying the input array, return the new length of the
/// array.
///
/// You must write an algorithm that uses only constant extra space.
struct Solution;

impl Solution {

    fn write_compressed(chars: &mut Vec<char>, i: &mut usize, last: char, count: usize) {
        chars[*i] = last;
        *i += 1;

        if count > 1 {
            let s = count.to_string();
            for c in s.chars() {
                chars[*i] = c;
                *i += 1;
            }
        }
    }

    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();

        let mut i = 0;

        let mut last = ' ';
        let mut count = 0;

        for j in 0..n {
            let letter = chars[j];
            if j == 0 {
                last = letter;
                count = 1;
            } else if letter == last {
                count += 1;
            } else {
                Self::write_compressed(chars, &mut i, last, count);

                last = letter;
                count = 1;
            }
        }
        Self::write_compressed(chars, &mut i, last, count);

        i as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 6);
        assert_eq!(chars.iter().take(6).collect::<String>(), "a2b2c3");
    }

    #[test]
    fn example_2() {
        let mut chars = vec!['a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 1);
        assert_eq!(chars.iter().take(1).collect::<String>(), "a");
    }

    #[test]
    fn example_3() {
        let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 4);
        assert_eq!(chars.iter().take(4).collect::<String>(), "ab12");
    }

    #[test]
    fn first_repeated() {
        let mut chars = vec!['z', 'z', 'z', 'z', 'z', 'z', 'z', 'z', 'z', 'z'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 3);
        assert_eq!(chars.iter().take(3).collect::<String>(), "z10");
    }

}
