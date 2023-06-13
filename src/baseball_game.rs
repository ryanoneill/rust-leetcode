/// You are keeping the scores for a baseball game with strange rules. At the beginning of the
/// game, you start with an empty record.
///
/// You are given a list of strings `operations`, where `operations[i]` is the `ith` operation you
/// must apply to the record and is one of the following:
///
/// * An integer `x`.
///   * Record a new score of `x`.
///
/// * `'+'`.
///   * Record a new score that is the sum of the previous two scores.
///
/// * `'D'`
///   * Record a new score that is the double of the previous score.
///
/// * `'C'`
///   * Invalidate the previous score, removing it from the record.
///
/// Return the sum of all the scores on the record after applying all the operations.
struct Solution;

impl Solution {

    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        println!("Operations = {:?}", operations);
        for op in operations {
            if op == "+" {
                let value2 = stack.pop();
                let value1 = stack.pop();
                match (value2, value1) {
                    (Some(val2), Some(val1)) => {
                        stack.push(val1);
                        stack.push(val2);
                        stack.push(val1 + val2);
                    }
                    _ => { }
                }
            } else if op == "D" {
                let value = stack.pop();
                match value {
                    Some(val) => {
                        stack.push(val);
                        stack.push(val * 2);
                    }
                    None => { }
                }

            } else if op == "C" {
                stack.pop();
            } else if let Ok(number) = op.parse::<i32>() {
                stack.push(number);
            }
            println!("Op = {:?}", op);
            println!("Stack = {:?}", stack);
        } 

        stack.into_iter().sum()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let ops = vec!["5","2","C","D","+"];
        let ops = ops.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::cal_points(ops);
        assert_eq!(result, 30);
    }

    #[test]
    fn example_2() {
        let ops = vec!["5", "-2", "4", "C", "D", "9", "+", "+"];
        let ops = ops.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::cal_points(ops);
        assert_eq!(result, 27);
    }

    #[test]
    fn example_3() {
        let ops = vec!["1", "C"];
        let ops = ops.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::cal_points(ops);
        assert_eq!(result, 0);
    }

}
