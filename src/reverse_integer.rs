/// Given a signed 32-bit integer `x`, return `x` with its digits reversed. If
/// reversing `x` causes the value to go outside the 32-bit integer range
/// `[-2^31, 2^31 - 1]`, then return `0`.
///
/// Assume the environment does not allow you to store 64-bit integers (signed
/// or unsigned).
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;
        let x = x.to_string();
        let mut stack = Vec::new();
        for c in x.chars() {
            match c {
                '1' => stack.push(1),
                '2' => stack.push(2),
                '3' => stack.push(3),
                '4' => stack.push(4),
                '5' => stack.push(5),
                '6' => stack.push(6),
                '7' => stack.push(7),
                '8' => stack.push(8),
                '9' => stack.push(9),
                '0' => stack.push(0),
                _ => {} // ignore '-' also
            }
        }

        let ten: i32 = 10;
        let mut result: i32 = 0;
        let mut place = stack.len() as u32;
        while !stack.is_empty() {
            place -= 1;
            let digit = stack.pop().unwrap();
            if digit != 0 {
                let power = ten.checked_pow(place);
                let piece = power.and_then(|p| p.checked_mul(digit));
                let check = piece.and_then(|p| result.checked_add(p));
                if check.is_some() {
                    result = check.unwrap();
                } else {
                    result = 0;
                    break;
                }
            }
        }
        // Not possible to reverse and get i32::MAX or i32::MIN
        // so this one digit difference in i32::MAX and i32::MIN is irrelevant here.
        if is_negative {
            result *= -1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let x = 123;
        let result = Solution::reverse(x);
        assert_eq!(result, 321);
    }

    #[test]
    fn example_2() {
        let x = -123;
        let result = Solution::reverse(x);
        assert_eq!(result, -321);
    }

    #[test]
    fn example_3() {
        let x = 120;
        let result = Solution::reverse(x);
        assert_eq!(result, 21);
    }

    #[test]
    fn same_reversed() {
        let x = 1000000001;
        let result = Solution::reverse(x);
        assert_eq!(result, x);
    }

    #[test]
    fn minimum() {
        let x = i32::MIN;
        let result = Solution::reverse(x);
        assert_eq!(result, 0);
    }

    #[test]
    fn maximum() {
        let x = i32::MAX;
        let result = Solution::reverse(x);
        assert_eq!(result, 0);
    }
}
