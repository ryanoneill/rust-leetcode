/// Given an integer `n`, return `true` if it is a power of four. Otherwise, return `false`.
///
/// An integer `n` is a power of four, if there exists an integer `x` such that `n == 4^x`.
struct Solution;

impl Solution {

    // -2^31 <= n <= 2^31 - 1
    // 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
    pub fn is_power_of_four(n: i32) -> bool {
        let mut result = false;
        if n < 1 {
            result = false;
        } else {
            let mut value = 1;
            let mut i = 0;
            while n > value {
                i += 1;
                if i > 15 {
                    break;
                } else {
                    value *= 4;
                }
            }
            if n == value {
                result = true;
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
        let n = 16;
        let result = Solution::is_power_of_four(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let result = Solution::is_power_of_four(n);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let n = 1;
        let result = Solution::is_power_of_four(n);
        assert!(result);
    }

    #[test]
    fn example_4() {
        let n = 0;
        let result = Solution::is_power_of_four(n);
        assert!(!result);
    }

    #[test]
    fn example_5() {
        let n = 256;
        let result = Solution::is_power_of_four(n);
        assert!(result);
    }

    #[test]
    fn example_6() {
        let n = 1073741824;
        let result = Solution::is_power_of_four(n);
        assert!(result);
    }

    #[test]
    fn example_7() {
        let n = i32::MAX;
        let result = Solution::is_power_of_four(n);
        assert!(!result);
    }

}
