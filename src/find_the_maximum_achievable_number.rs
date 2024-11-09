/// Given two integers, `num` and `t`. A number is achievable if it can become equal to `num` after
/// applying the following operations:
///
/// * Increase or decrease the number by `1`, and simultaneously increase or decrease the `num` by
/// `1`.
///
/// Return the maximum achievable number after applying the operation at most `t` times.
struct Solution;

impl Solution {

    fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + (2 * t)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 4;
        let t = 1;
        let result = Solution::the_maximum_achievable_x(num, t);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let num = 3;
        let t = 2;
        let result = Solution::the_maximum_achievable_x(num, t);
        assert_eq!(result, 7);
    }

}
