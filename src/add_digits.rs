/// Given an integer `num`, repeatedly add all its digits until the result
/// has only one digit, and return it.
struct Solution;

impl Solution {

    pub fn add_digits(num: i32) -> i32 {
        let mut result = 0;
        let mut current = num;

        while current > 9 {
            result += 1;
            let s = current.to_string();
            current = s.split("")
                .filter(|s| *s != "")
                .map(|d| { d.parse::<i32>().unwrap() })
                .sum();
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num = 38;
        let result = Solution::add_digits(num);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let num = 0;
        let result = Solution::add_digits(num);
        assert_eq!(result, 0);
    }

}
