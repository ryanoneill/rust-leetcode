/// Given an integer `n`, return `true` if and only if it is an Armstrong number.
///
/// The `k`-digit number `n` is an Armstrong number if and only if the `kth` power of each digit
/// sums to `n`.
struct Solution;

impl Solution {

    fn to_digits(n: i32) -> Vec<i32> {
        let mut current = n;
        let mut result = vec![];

        while current > 0 {
            let digit = current % 10;
            current = current / 10;
            result.push(digit);
        }

        result
    }

    // 1 <= n <= 10^8
    pub fn is_armstrong(n: i32) -> bool {
        let digits = Self::to_digits(n);
        let count = digits.len();
        let pow = count as u32;

        let n = n as u64;
        let mut sum: u64 = 0;

        for digit in &digits {
            let piece = digit.pow(pow);
            sum += piece as u64;
        }

        sum == n
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 153;
        let result = Solution::is_armstrong(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 123;
        let result = Solution::is_armstrong(n);
        assert!(!result);
    }

}
