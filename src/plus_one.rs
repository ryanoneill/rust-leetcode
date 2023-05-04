/// You are given a large integer represented as an integer array `digits`,
/// where each `digits[i]` is the `ith` digit of the integer. The digits are
/// ordered from most significant to least significant in left-to-right order.
/// The large integer does not contain any leading `0`'s.
///
/// Increment the large integer by one and return the resulting array of
/// digits.
struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut result = Vec::new();
        let mut carry = false;
        let mut digit;

        for i in 0..n {
            digit = digits[n - i - 1];
            if i == 0 || carry {
                digit += 1;
                carry = false;
            }
            if digit > 9 {
                digit -= 10;
                carry = true;
            }
            result.push(digit);
        }
        if carry {
            result.push(1);
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn example_2() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_3() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 0]);
    }

    #[test]
    fn real_world() {
        let digits = vec![8, 9, 9, 9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![9, 0, 0, 0]);
    }
}
