/// Given an integer `num`, return the number of steps to reduce it to zero.
///
/// In one step, if the current number is even, you have to divide it by `2`,
/// otherwise, you have to subtract `1` from it.
struct Solution;

impl Solution {

    pub fn number_of_steps(num: i32) -> i32 {
        let mut steps: i32 = 0;
        let mut current: i32 = num;

        while current != 0 {
            steps += 1;
            if current % 2 == 0 {
                current /= 2;
            } else {
                current -= 1;
            }
        }

        steps
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 14;
        let result = Solution::number_of_steps(num);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let num = 8;
        let result = Solution::number_of_steps(num);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let num = 123;
        let result = Solution::number_of_steps(num);
        assert_eq!(result, 12);
    }

}
