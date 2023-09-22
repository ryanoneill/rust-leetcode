/// Given a signed 32-bit integer `x`, return `x` with its digits reversed. If
/// reversing `x` causes the value to go outside the 32-bit integer range
/// `[-2^31, 2^31 - 1]`, then return `0`.
///
/// Assume the environment does not allow you to store 64-bit integers (signed
/// or unsigned).
struct Solution;

impl Solution {

    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;

        if x != i32::MIN {
            let is_negative = x < 0;

            let mut value = x;
            if is_negative {
                value *= -1;
            }

            while value > 0 {
                let digit = value % 10;
                value /= 10;

                if result > i32::MAX / 10 {
                    result = 0;
                    break;
                } else {
                    result *= 10;
                }
                if result > i32::MAX - digit {
                    result = 0;
                    break;
                } else {
                    result += digit;
                }
            }

            if is_negative {
                result *= -1;
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
