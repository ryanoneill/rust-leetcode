/// You are given a positive integer `num` consisting only of digits `6` and
/// `9`.
///
/// Return the maximum number you can get by changing at most one digit (`6`
/// becomes `9`, and `9` becomes `6`).
struct Solution;

impl Solution {

    fn to_digits(num: i32) -> Vec<i32> {
        let mut num = num;
        let mut result = Vec::new();

        while num > 0 {
            let digit = num % 10;
            result.push(digit);
            num /= 10;
        }

        result.into_iter().rev().collect()
    }

    fn from_digits(digits: Vec<i32>) -> i32 {
        let n = digits.len();
        let mut result = 0;
        for i in 0..n {
            let digit = digits[i];
            result *= 10;
            result += digit;
        }

        result
    }

    fn first_six_to_nine(digits: &mut Vec<i32>) {
        let n = digits.len();
        for i in 0..n {
            let digit = digits[i];
            if digit == 6 {
                digits[i] = 9;
                break;
            }
        }
    }

    pub fn maximum69_number(num: i32) -> i32 {
        let mut digits = Self::to_digits(num);
        Self::first_six_to_nine(&mut digits);
        Self::from_digits(digits)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 9669;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9969);
    }

    #[test]
    fn example_2() {
        let num = 9996;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9999);
    }

    #[test]
    fn example_3() {
        let num = 9999;
        let result = Solution::maximum69_number(num);
        assert_eq!(result, 9999);
    }

}
