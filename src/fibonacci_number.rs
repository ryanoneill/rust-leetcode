/// The Fibonacci numbers, commonly denoted as `F(n)` form a sequence, called
/// the Fibonacci sequence, such that each number is the sum of the two
/// preceding ones, starting from `0` and `1`. That is
///
/// `
/// F(0) = 0, F(1) = 1
/// F(n) = F(n-1) + F(n-2), for n > 1.
/// `
///
/// Given `n`, calculate `F(n)`.
struct Solution;

impl Solution {

    pub fn fib(n: i32) -> i32 {
        if n == 0 { 0 }
        else if n == 1 { 1 }
        else {
            let mut minus_two = 0;
            let mut minus_one = 1;
            let mut result = 0;

            for _ in 2..=n {
                result = minus_two + minus_one;
                minus_two = minus_one;
                minus_one = result;
            }

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 2;
        let result = Solution::fib(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let result = Solution::fib(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let n = 4;
        let result = Solution::fib(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn zero() {
        let n = 0;
        let result = Solution::fib(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn one() {
        let n = 1;
        let result = Solution::fib(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn largest() {
        let n = 30;
        let result = Solution::fib(n);
        assert_eq!(result, 832040);
    }

}
