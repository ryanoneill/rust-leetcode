/// Given a string `s`, return the length of the longest substring that contains at most two
/// distinct characters.
struct Solution;

impl Solution {

    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let letters: Vec<char> = s.chars().collect();

        let mut letter_one = ' ';
        let mut letter_one_last = 0;

        let mut letter_two = ' ';
        let mut letter_two_last = 0;

        let mut result = 0;
        let mut count = 0;

        let n = letters.len();
        for i in 0..n {
            let letter = letters[i];
            if letter == letter_one {
                letter_one_last = i;
                count += 1;
            } else if letter == letter_two {
                letter_two_last = i;
                count += 1;
            } else if letter_one == ' ' {
                letter_one = letter;
                letter_one_last = i;
                count += 1;
            } else if letter_two == ' ' {
                letter_two = letter;
                letter_two_last = i;
                count += 1;
            } else if letter_one_last < letter_two_last {
                count = (letter_two_last - letter_one_last) as i32 + 1;
                letter_one = letter;
                letter_one_last = i;
            } else {
                count = (letter_one_last - letter_two_last) as i32 + 1;
                letter_two = letter;
                letter_two_last = i;
            }
            result = result.max(count);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("eceba");
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let s = str!("ccaabbb");
        let result = Solution::length_of_longest_substring_two_distinct(s);
        assert_eq!(result, 5);
    }

}

