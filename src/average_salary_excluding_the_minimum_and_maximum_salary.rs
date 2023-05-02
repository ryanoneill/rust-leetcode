use std::cmp::max;
use std::cmp::min;

/// You are given an array of unique integers `salary` where `salary[i]` is the
/// salary of the `ith` employee.
///
/// Return the average salary of employees excluding the minimum and maximum
/// salary. Answers within `10^-5` of the actual answer will be accepted.
struct Solution;

impl Solution {

    pub fn average(salary: Vec<i32>) -> f64 {
        let n = salary.len();
        let mut min_salary = i32::max_value();
        let mut max_salary = i32::min_value();
        let mut sum = 0;

        for s in salary {
            min_salary = min(min_salary, s);
            max_salary = max(max_salary, s);
            sum += s;
        }

        sum -= min_salary;
        sum -= max_salary;
        sum as f64 / (n - 2) as f64
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let salary = vec![4000, 3000, 1000, 2000];
        let result = Solution::average(salary);
        assert_eq!(result, 2500.00000);
    }

    #[test]
    fn example_2() {
        let salary = vec![1000, 2000, 3000];
        let result = Solution::average(salary);
        assert_eq!(result, 2000.00000);
    }

}
