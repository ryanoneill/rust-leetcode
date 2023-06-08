/// Given a string containing digits from `2-9` inclusive, return all possible
/// letter combinations that the number could represent. Return the answer in
/// any order.
///
/// A mapping of digits to letters (just like on the telephone buttons) is
/// given below. Note that 1 does not map to any letters.
struct Solution;

impl Solution {

    fn digit_to_letters(digit: char) -> Vec<char> {
        let mut result = vec![];
        match digit {
            '2' => { result = vec!['a', 'b', 'c']; }
            '3' => { result = vec!['d', 'e', 'f']; }
            '4' => { result = vec!['g', 'h', 'i']; }
            '5' => { result = vec!['j', 'k', 'l']; }
            '6' => { result = vec!['m', 'n', 'o']; }
            '7' => { result = vec!['p', 'q', 'r', 's']; }
            '8' => { result = vec!['t', 'u', 'v']; }
            '9' => { result = vec!['w', 'x', 'y', 'z']; }
            _   => { }
        }
        result
    }

    fn add_digit(answers: Vec<Vec<char>>, digit: char) -> Vec<Vec<char>> {
        let letters = Self::digit_to_letters(digit);
        let n = answers.len();
        let mut result = Vec::with_capacity(n * letters.len());
        for letter in letters {
            for answer in &answers {
                let mut cloned = answer.clone();
                cloned.push(letter);
                result.push(cloned);
            }
        }

        result
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let n = digits.len();
        if n == 0 { vec![] }
        else {
            let mut result = vec![vec![]];
            for digit in digits.chars() {
                result = Self::add_digit(result, digit)
            }
            result.into_iter().map(|s| s.into_iter().collect()).collect()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let digits = "23".to_string();
        let mut result = Solution::letter_combinations(digits);
        result.sort();
        assert_eq!(result, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    #[test]
    fn example_2() {
        let digits = "".to_string();
        let mut result = Solution::letter_combinations(digits);
        result.sort();
        let empty: Vec<String> = vec![];
        assert_eq!(result, empty);
    }

}
