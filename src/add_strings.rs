/// Given two non-negative integers, `num1` and `num2` represented as string, return the sum of
/// `num1` and `num2` as a string.
///
/// You must solve the problem without using any built in library for handling large integers (such
/// as `BigInteger`)
struct Solution;

impl Solution {

    fn char_to_digit(value: char) -> i32 {
        match value {
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

    fn digit_to_char(value: i32) -> char {
        match value {
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

    pub fn add_strings(num1: String, num2: String) -> String {
        let result;

        if num1 == "0" {
            result = num2;
        } else if num2 == "0" {
            result = num1;
        } else {
            let num1_digits: Vec<char> = num1.chars().rev().collect();
            let num1_len = num1_digits.len();
            let num2_digits: Vec<char> = num2.chars().rev().collect();
            let num2_len = num2_digits.len();

            let n = num1_len.max(num2_len);

            let mut results = Vec::new();
            let mut carry = false;

            let mut value1;
            let mut value2;

            for i in 0..n {
                if num1_len > i {
                    value1 = Self::char_to_digit(num1_digits[i]);
                } else {
                    value1 = 0;
                }

                if num2_len > i {
                    value2 = Self::char_to_digit(num2_digits[i]);
                } else {
                    value2 = 0;
                }

                let mut part = 0;
                if carry {
                    part += 1;
                    carry = false;
                }
                part += value1;
                part += value2;

                if part >= 10 {
                    part -= 10;
                    carry = true;
                }
                results.push(Self::digit_to_char(part));
            }

            if carry {
                results.push('1');
            }

            result = results.into_iter().rev().collect();
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        let result = Solution::add_strings(num1, num2);
        assert_eq!(result, "134");
    }

    #[test]
    fn example_2() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        let result = Solution::add_strings(num1, num2);
        assert_eq!(result, "533");
    }

    #[test]
    fn example_3() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        let result = Solution::add_strings(num1, num2);
        assert_eq!(result, "0");
    }

}

// 1. Write Down the Problem
//
// 2. Clarify the Problem Space
// * 1 <= num.length
// * 1 <= num2.length
// * num1.length <= 10^4
// * num2.length <= 10^4
// * Input: 2 Strings / Output: 1 String
//
// 3. Write Down the Test Cases
// * Input: "123", "142" / Output: "265"
// * Input: "0", "438" / Output: "438"
// * Input: "987", "0" / Output: "987"
// * Input: "0", "0" / Output: "0"
// * Input: "123", "129" / Output: "252"
// * Input: "998", "50" / Output: "1048"
//
// 4. Describe and Write Down the Algorithm
// * If num1 == 0, return num2
// * If num2 == 0, return num1
// * Convert num1 (reversed) to collection of digits
// * Convert num2 (reversed) to collection of digits
// * Iterate through vectors based on longer one, adding together v[i] for each plus any carry from
//   previous operation. Push onto result vector.
//   ** If carry is true, add one and set carry to false.
//   ** If the value is greater than 9, then subtract 10 and set carry to true.
// * If there is a carry remaining, add a one to the end of the result vector.
// * Convert the reversed result vector into a string and return it.
