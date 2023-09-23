/// Given two non-negative integers `num1` and `num2` represented as strings, return the product of
/// `num1` and `num2`, also represented as a string.
///
/// Note: You must not use any built-in BigInteger library or conver the inputs to integer
/// directly.
struct Solution;

impl Solution {

    fn to_char(digit: i32) -> char {
        match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => '0',
        }
    }

    fn to_digit(c: char) -> i32 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _   => 0,
        }
    }

    fn to_reversed_num_vec(num: String) -> Vec<i32> {
        let mut result = Vec::new();

        for c in num.chars().rev() {
            let digit = Self::to_digit(c);
            result.push(digit);
        }

        result
    }

    fn max_len(parts: &Vec<Vec<i32>>) -> usize {
        let mut result = 0;
        for part in parts {
            result = result.max(part.len());
        }
        result
    }

    fn multiply_parts(digits1: Vec<i32>, digits2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut parts = Vec::new();
        let n = digits2.len();
        for i in 0..n {
            let mut part = Vec::new();
            let digit2 = digits2[i];
            if digit2 == 0 {
                part.push(0);
            } else {
                for _ in 1..=i {
                    part.push(0);
                }
                let mut carry = 0;

                for digit1 in &digits1 {
                    let whole = digit2 * *digit1 + carry;
                    carry = whole / 10;
                    part.push(whole % 10);
                }

                if carry != 0 {
                    part.push(carry);
                }
            }
            parts.push(part);
        }

        parts
    }

    fn add_parts(parts: &Vec<Vec<i32>>) -> String {
        let m = Self::max_len(parts);
        let mut results = Vec::new();
        let mut carry = 0;
        for i in 0..m {
            let mut whole = carry;
            for part in parts {
                if part.len() > i {
                    whole += part[i];
                }
            }
            carry = whole / 10;
            results.push(whole % 10);
        }
        if carry != 0 {
            results.push(carry);
        }

        let mut result = String::new();
        for digit in results.iter().rev() {
            result.push(Self::to_char(*digit));
        }
        result
    }

    pub fn multiply(num1: String, num2: String) -> String {
        let result;

        if num1 == "0" || num2 == "0" {
            result = "0".to_string();
        } else {
            let digits1 = Self::to_reversed_num_vec(num1);
            let digits2 = Self::to_reversed_num_vec(num2);

            let parts = Self::multiply_parts(digits1, digits2);
            result = Self::add_parts(&parts);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "6");
    }

    #[test]
    fn example_2() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "56088");
    }

    #[test]
    fn example_3() {
        let num1 = "99999".to_string();
        let num2 = "54".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "5399946");
    }

    #[test]
    fn example_4() {
        let num1 = "9133".to_string();
        let num2 = "0".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "0");
    }

    #[test]
    fn example_5() {
        let num1 = "0".to_string();
        let num2 = "52".to_string();
        let result = Solution::multiply(num1, num2);
        assert_eq!(result, "0");
    }

}
