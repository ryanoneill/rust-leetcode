/// Given a string `s` of zeros and ones, return the maximum score after splitting the string into
/// two non-empty substrings (i.e. left substring and right substring).
///
/// The score after splitting a string is the number of zeros in the left substring plus the number
/// of ones in the right substring.
struct Solution;

impl Solution {

    pub fn max_score(s: String) -> i32 {
        let digits: Vec<char> = s.chars().collect();
        let n = digits.len();

        let mut zeros = 0;
        let mut ones = 0;

        let mut current;
        let mut result = 0;

        for i in 0..n {
            let value = digits[i];
            if value == '1' {
                ones += 1;
            }
        }

        for i in 0..n-1 {
            let value = digits[i];
            if value == '0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            current = zeros + ones;
            result = result.max(current);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("011101");
        let result = Solution::max_score(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let s = str!("00111");
        let result = Solution::max_score(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_3() {
        let s = str!("1111");
        let result = Solution::max_score(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_4() {
        let s = str!("0000");
        let result = Solution::max_score(s);
        assert_eq!(result, 3);
    }

}
