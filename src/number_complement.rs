/// The complement of an integer is the integer you get when you flip all the `0`'s to `1`'s and
/// all the `1`'s to `0`'s in its binary representation.
///
/// * For example, the integer `5` is `"101"` in binary and its complement is `"010"` which is the
///   integer `2`.
///
/// Given an integer `num`, return its complement.
struct Solution;

impl Solution {

    pub fn find_complement(num: i32) -> i32 {
        let zeros = num.leading_zeros();
        let mut result = !num;
        result = result << zeros;
        result = result >> zeros;
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 5;
        let result = Solution::find_complement(num);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let num = 1;
        let result = Solution::find_complement(num);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let num = i32::MAX;
        let result = Solution::find_complement(num);
        assert_eq!(result, 0);
    }

}
