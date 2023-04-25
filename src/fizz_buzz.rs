/// Given an integer `n`, return a string array `answer` (1-indexed) where:
///
/// * `answer[i] == "FizzBuzz"` if `i` is divisble for `3` and `5`.
/// * `answer[i] == "Fizz"` if `i` is divisible by `3`.
/// * `answer[i] == "Buzz"` if `i` is divisible by `5`.
/// * `answer[i] == i` (as a string) if none of the above conditions are true.
struct Solution;

impl Solution {

    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                result.push(String::from("FizzBuzz"));
            } else if i % 3 == 0 {
                result.push(String::from("Fizz"));
            } else if i % 5 == 0 {
                result.push(String::from("Buzz"));
            } else {
                result.push(i.to_string());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let result = Solution::fizz_buzz(3);
        assert_eq!(result, vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn example_2() {
        let result = Solution::fizz_buzz(5);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn example_3() {
        let result = Solution::fizz_buzz(15);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7",
            "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]);
    }

}
