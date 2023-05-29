use std::collections::HashSet;

struct Solution;

impl Solution {
    
    pub fn is_power_of_three(n: i32) -> bool {
        match n {
            _ if n < 0 => false,
            0 => false,
            1 => true,
            2 => false,
            3 => true,
            _ => {
                let mut result = true;
                let mut n = n;

                while n > 3 {
                    if n % 3 == 0 {
                        n /= 3;
                    } else {
                        result = false;
                        break;
                    }
                }
                result = result && n == 3;

                result
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 27;
        let result = Solution::is_power_of_three(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 0;
        let result = Solution::is_power_of_three(n);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let n = -1;
        let result = Solution::is_power_of_three(n);
        assert!(!result);
    }

    #[test]
    fn three() {
        let n = 3;
        let result = Solution::is_power_of_three(n);
        assert!(result);
    }

    #[test]
    fn six() {
        let n = 6;
        let result = Solution::is_power_of_three(n);
        assert!(!result);
    }

}
