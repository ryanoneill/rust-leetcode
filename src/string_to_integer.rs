#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Ord)]
enum State {
    Whitespace,
    Sign,
    Digits
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Ord)]
enum Sign {
    Positive,
    Negative
}

/// Implement the `myAtoi(string s)` function, which converts a string to a
/// 32-bit signed integer (similar to C/C++'s `atoi` function).
///
/// The algorithm for `myAtoi(string s)` is as follows:
///
/// Read in and ignore any leading whitespace.
///
/// Check if the next character (if not already at the end of the string) is
/// `'-'` or `'+'`. Read this character in if it is either. This determines
/// if the final result is negative or positive respectively. Assume the result
/// is positive if neight is present.
///
/// Read in next the characters until the next non-digit character or the end
/// of the input is reached. The rest of the string is ignored.
///
/// Convert these digits into an integer (i.e. `"123" -> 123`, `"0032" -> 32`).
/// If no digits were read, then the integer is `0`. Change the sign as
/// necessary (from step 2).
///
/// If the integer is out of the 32-bit signed integer range
/// `[-2^31, 2^31 - 1]`, then clamp the integer so that it remains in the
/// range. Specifically, integers less than `-2^31` should be clamped to
/// `2^31`, and integers greater than `2^31 - 1` should be clamped to
/// `2^31 - 1`.
///
/// Return the integer as the final result.
///
/// Note:
/// * Only the space character is considered a whitespace character.
/// * Do not ignore any characters other than the leading whitespace or the
///   rest of the string after the digits.
struct Solution;

impl Solution {

    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars();
        let mut state = State::Whitespace;

        let mut sign = Sign::Positive;
        let mut digits: Vec<i32> = Vec::new();
        let result;

        loop {
            let current = chars.next();
            if current.is_none() { break; }
            else {
                let current = current.unwrap();
                if state == State::Whitespace {
                    if current != ' ' {
                        state = State::Sign;
                    }
                }
                if state == State::Sign {
                    if current == '+' {
                        sign = Sign::Positive;
                        state = State::Digits;
                        continue;
                    } else if current == '-' {
                        sign = Sign::Negative;
                        state = State::Digits;
                        continue;
                    } else {
                        state = State::Digits;
                    }
                }
                if state == State::Digits {
                    match current {
                        '1' => digits.push(1),
                        '2' => digits.push(2),
                        '3' => digits.push(3),
                        '4' => digits.push(4),
                        '5' => digits.push(5),
                        '6' => digits.push(6),
                        '7' => digits.push(7),
                        '8' => digits.push(8),
                        '9' => digits.push(9),
                        '0' => digits.push(0),
                        _   => break,
                    }
                }
            }
        }
        let mut working = 0 as i64;
        let ten: i64 = 10;
        for i in 0..digits.len() {
            let digit = digits.pop().unwrap() as i64;
            if digit != 0 {
                let power = ten.checked_pow(i as u32);
                let piece = power.map(|p| p * digit);
                working = piece.and_then(|p| working.checked_add(p)).unwrap_or(i64::MAX);
            }
        }
        if sign == Sign::Negative {
            working *= -1;
        }
        if working < i32::MIN as i64 {
            result = i32::MIN;
        } else if working > i32::MAX as i64 {
            result = i32::MAX;
        } else {
            result = working as i32;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "42".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, 42);
    }

    #[test]
    fn example_2() {
        let s = "    -42".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, -42);
    }

    #[test]
    fn example_3() {
        let s = "4193 with words".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, 4193);
    }

    #[test]
    fn letters() {
        let s = "abcd".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn double_sign() {
        let s = "--42".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn really_large() {
        let s = "9223372036854775808".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, i32::MAX);
    }

    #[test]
    fn really_small() {
        let s = "-9223372036854775808".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, i32::MIN);
    }

    #[test]
    fn real_world_1() {
        let s = "18446744073709551617".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, i32::MAX);
    }

    #[test]
    fn real_world_2() {
        let s = "-000000000000000000000000000000000000000000000000001".to_string();
        let result = Solution::my_atoi(s);
        assert_eq!(result, -1);
    }


}
