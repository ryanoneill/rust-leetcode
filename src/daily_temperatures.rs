/// Given an array of integers `temperatures` represents the daily
/// temperatures, return an array `answer` such that `answer[i]` is the number
/// of days you have to wait after the `ith` day to get a warmer temperature.
/// If there is no future day for which this is possible, keep answer[i] == 0
/// instead.
struct Solution;

impl Solution {

    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut result = vec![0; n];

        for i in 0..n {
            let current = temperatures[i];

            while !stack.is_empty() {
                let n = stack.len();
                let index = stack[n-1];
                let previous = temperatures[index];
                if current > previous {
                    result[index] = (i - index) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(i);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = Solution::daily_temperatures(temperatures);
        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let result = Solution::daily_temperatures(temperatures);
        assert_eq!(result, vec![1, 1, 1, 0]);
    }

    #[test]
    fn example_3() {
        let temperatures = vec![30, 60, 90];
        let result = Solution::daily_temperatures(temperatures);
        assert_eq!(result, vec![1, 1, 0]);
    }
}
