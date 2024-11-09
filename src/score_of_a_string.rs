/// You are given a string `s`. The score of a string is defined as the sum of the absolute
/// difference between the ASCII values of adjacent characters.
/// 
/// Return the score of `s`.
struct Solution;

impl Solution {

    pub fn score_of_string(s: String) -> i32 {
        let mut result = 0;
        let mut last = ' ';

        for c in s.chars() {
            if last != ' ' {
                let diff = ((last as i32) - (c as i32)).abs();
                result += diff;
            }
            last = c;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("hello");
        let result = Solution::score_of_string(s);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let s = str!("zaz");
        let result = Solution::score_of_string(s);
        assert_eq!(result, 50);
    }

}
