/// Given an array of integers `temperatures` represents the daily
/// temperatures, return an array `answer` such that `answer[i]` is the number
/// of days you have to wait after the `ith` day to get a warmer temperature.
/// If there is no future day for which this is possible, keep answer[i] == 0
/// instead.
struct Solution;

impl Solution {
    fn has_lower_temp(temperatures: &Vec<i32>, stack: &Vec<usize>, temp: i32) -> bool {
        if stack.is_empty() {
            false
        } else {
            let last_index = stack.iter().last().unwrap();
            let last_temp = temperatures[*last_index];
            last_temp < temp
        }
    }

    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result: Vec<i32> = vec![0; temperatures.len()];

        for (i, temp) in temperatures.iter().enumerate() {
            while Self::has_lower_temp(&temperatures, &stack, *temp) {
                let j: usize = stack.pop().unwrap();
                result[j] = i as i32 - j as i32;
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
