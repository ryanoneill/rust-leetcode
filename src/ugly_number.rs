/// An ugly number is a positive integer whose prime factors are limited to
/// `2`, `3`, and `5`.
struct Solution;

impl Solution {

    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        loop {
            if n == 0 {
                break;
            } else if n % 5 == 0 {
                n = n / 5;
            } else if n % 3 == 0 {
                n = n / 3;
            } else if n % 2 == 0 {
                n = n / 2;
            } else {
                break;
            }
        }
        n == 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 6;
        let result = Solution::is_ugly(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::is_ugly(n);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let n = 14;
        let result = Solution::is_ugly(n);
        assert!(!result);
    }

    #[test]
    fn negative() {
        let n = -10;
        let result = Solution::is_ugly(n);
        assert!(!result);
    }

    #[test]
    fn zero() {
        let n = 0;
        let result = Solution::is_ugly(n);
        assert!(!result);
    }

}
