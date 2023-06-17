/// You are given an array of strings `tokens` that represents an arithmetic expression in a
/// Reverse Polish Notation.
///
/// Evaluate the expression. Return an integer that represents the value of the expression.
///
/// Note that:
///
/// * The valid operators are `'+'`, `'-'`, `'*'`, and `'/'`.
///
/// * Each operand may be an integer or another expression.
///
/// * The division between two integers always truncates toward zero.
///
/// * There will not be any division by zero.
///
/// * The input represents a valid arithmetic expression in a reverse polish notation.
///
/// * The answer and all the intermediate calculations can be represented in a 32-bit integer.
struct Solution;

impl Solution {

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    stack.push(arg1 + arg2);
                }
                "-" => {
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    stack.push(arg1 - arg2);
                }
                "*" => {
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    stack.push(arg1 * arg2);
                }
                "/" => {
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    stack.push(arg1 / arg2);
                }
                _ => {
                    let value = token.parse::<i32>().unwrap();
                    stack.push(value);
                }
            }

        }

        stack.pop().unwrap()
    }
    
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        let tokens = tokens.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::eval_rpn(tokens);
        assert_eq!(result, 9);
    }

    #[test]
    fn example_2() {
        let tokens = vec!["4", "13", "5", "/", "+"];
        let tokens = tokens.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::eval_rpn(tokens);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        let tokens = vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"];
        let tokens = tokens.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::eval_rpn(tokens);
        assert_eq!(result, 22);
    }

}
