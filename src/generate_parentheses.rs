/// Given `n` pairs of parentheses, write a function to generate all combinations of well-formed
/// parentheses.
struct Solution;

impl Solution {

    fn backtrack(results: &mut Vec<String>, stack: &mut Vec<char>, n: usize, open: usize, closed: usize) {
        if open == closed && open == n {
            results.push(stack.iter().collect());
        } else {
            if open < n {
                stack.push('(');
                Self::backtrack(results, stack, n, open + 1, closed);
                stack.pop();
            } 
            if closed < open {
                stack.push(')');
                Self::backtrack(results, stack, n, open, closed + 1);
                stack.pop();
            }
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results = Vec::new();
        let mut stack = Vec::new();
        let n = n as usize;

        Self::backtrack(&mut results, &mut stack, n, 0, 0);
        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        assert_eq!(result, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(result, vec!["()"]);
    }

    #[test]
    fn two() {
        let n = 2;
        let result = Solution::generate_parentheses(n);
        assert_eq!(result, vec!["(())", "()()"]);
    }

}
