/// Implement `pow(x,n)`, which calculates `x`, raised to the power `n` (i.e., `x^n`).
struct Solution;

impl Solution {

    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            i32::MIN => {
                1.0 / (x * Self::my_pow(x, i32::MAX))
            }
            value if value < 0 => {
                1.0 / Self::my_pow(x, -1 * n)
            }
            0 => 1.0,
            1 => x,
            2 => x * x,
            value if value % 2 == 0 => {
                let intermediate = Self::my_pow(x, n / 2);
                intermediate * intermediate
            }
            _ => {
                let intermediate = Self::my_pow(x, n / 2);
                intermediate * intermediate * x
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn within_delta(expected: f64, actual: f64) -> bool {
        (expected - actual).abs() < 0.00001
    }

    #[test]
    fn example_1() {
        let x = 2.00000;
        let n = 10;
        let result = Solution::my_pow(x, n);
        assert!(within_delta(result, 1024.00000));
    }

    #[test]
    fn example_2() {
        let x = 2.10000;
        let n = 3;
        let result = Solution::my_pow(x, n);
        assert!(within_delta(result, 9.26100));
    }

    #[test]
    fn example_3() {
        let x = 2.00000;
        let n = -2;
        let result = Solution::my_pow(x, n);
        assert!(within_delta(result, 0.25000));
    }

}
