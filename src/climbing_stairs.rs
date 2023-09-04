/// You are climbing a staircase. It takes `n` steps to reach the top.
///
/// Each time you can either climb `1` or `2` steps. In how many distinct ways
/// can you climb to the top?
struct Solution;

impl Solution {

    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => {
                let mut minus_two = 1;
                let mut minus_one = 2;
                let mut result = 0;

                for _ in 3..=n {
                    result = minus_one + minus_two;
                    minus_two = minus_one;
                    minus_one = result;
                }

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
        let n = 2;
        let result = Solution::climb_stairs(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let result = Solution::climb_stairs(n);
        assert_eq!(result, 3);
    }
}
