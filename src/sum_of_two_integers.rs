/// Given two integers `a` and `b`, return the sum of the two integers without using the operators
/// `+` and `-`.
struct Solution;

impl Solution {

    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;

        while b != 0 {
            let temp = (a & b) << 1;
            a = a ^ b;
            b = temp;
        }

        a
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let a = 1;
        let b = 2;
        let result = Solution::get_sum(a, b);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let a = 2;
        let b = 3;
        let result = Solution::get_sum(a, b);
        assert_eq!(result, 5);
    }

    #[test]
    fn first_negative() {
        let a = -101;
        let b = 55;
        let result = Solution::get_sum(a, b);
        assert_eq!(result, -46);
    }

    #[test]
    fn second_negative() {
        let a = 101;
        let b = -55;
        let result = Solution::get_sum(a, b);
        assert_eq!(result, 46);
    }

    #[test]
    fn both_negative() {
        let a = -101;
        let b = -55;
        let result = Solution::get_sum(a, b);
        assert_eq!(result, -156);
    }

}
