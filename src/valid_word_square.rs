/// Given an array of strings `words`, return `true` if it forms a valid word square.
///
/// A sequence of strings forms a valid word square if the `kth` row and column read the same
/// string, where `0 <= k < max(numRows, numColumns)`.
struct Solution;

impl Solution {

    fn get_char(words: &Vec<Vec<char>>, i: usize, j: usize) -> char {
        let mut result = ' ';
        let n = words.len();
        if i < n {
            let m = words[i].len();
            if j < m {
                result = words[i][j];
            }
        }
        result
    }

    fn find_max(words: &Vec<Vec<char>>) -> usize {
        let n = words.len();
        let mut result = n;
        for i in 0..n {
            let m = words[i].len();
            result = result.max(m);
        }
        result
    }

    pub fn valid_word_square(words: Vec<String>) -> bool {
        let mut result = true;

        let n = words.len();
        let mut square = Vec::with_capacity(n);
        for word in words {
            let letters: Vec<char> = word.chars().collect();
            square.push(letters);
        }

        let k = Self::find_max(&square);
        for i in 0..k {
            for j in i+1..k {
                let a = Self::get_char(&square, i, j);
                let b = Self::get_char(&square, j, i);
                if a != b {
                    result = false;
                    break;
                }
            }

            if !result {
                break;
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
        let words = vec![str!("abcd"), str!("bnrt"), str!("crmy"), str!("dtye")];
        let result = Solution::valid_word_square(words);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let words = vec![str!("abcd"), str!("bnrt"), str!("crm"), str!("dt")];
        let result = Solution::valid_word_square(words);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let words = vec![str!("ball"), str!("area"), str!("read"), str!("lady")];
        let result = Solution::valid_word_square(words);
        assert!(!result);
    }

    #[test]
    fn example_4() {
        let words = vec![str!("abcde"), str!("bnrt"), str!("crmy"), str!("dtye")];
        let result = Solution::valid_word_square(words);
        assert!(!result);
    }

    #[test]
    fn example_5() {
        let words = vec![str!("ball"), str!("asee"), str!("let"), str!("lep")];
        let result = Solution::valid_word_square(words);
        assert!(!result);
    }

}
