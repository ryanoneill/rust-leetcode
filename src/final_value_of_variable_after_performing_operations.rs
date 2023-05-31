/// There is a programming language with only four operations and one variable `X`:
///
/// * `++X` and `X++` increments the value of the variable `X` by `1`.
///
/// * `--X` and `X--` decrements the value of the variable `X` by `1`.
///
/// Initially, the value of `X` is `0`.
///
/// Given an array of strings `operations` containing a list of operations,
/// return the final value o `X` after performing all the operations.
struct Solution;

impl Solution {

    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;
        for op in operations {
            match op.as_str() {
                "++X" | "X++" => {
                    result += 1;
                }
                "--X" | "X--" => {
                    result -= 1;
                }
                _ => { }
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
        let operations = vec!["--X", "X++", "X++"].iter().map(|s| s.to_string()).collect();
        let result = Solution::final_value_after_operations(operations);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let operations = vec!["++X", "++X", "X++"].iter().map(|s| s.to_string()).collect();
        let result = Solution::final_value_after_operations(operations);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let operations = vec!["X++", "++X", "--X", "X--"].iter().map(|s| s.to_string()).collect();
        let result = Solution::final_value_after_operations(operations);
        assert_eq!(result, 0);
    }

}
