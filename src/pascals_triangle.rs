/// Given an integer `numRows`, return the first numRows of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
struct Solution;

impl Solution {

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut results: Vec<Vec<i32>> = vec![];

        for i in 0..n {
            if i == 0 {
                let result = vec![1];
                results.push(result);
            } else if i == 1 {
                let result = vec![1,1];
                results.push(result);
            } else {
                let previous: &Vec<i32> = &results[i-1];
                let m = previous.len();
                let mut result = vec![1; m+1];
                for j in 1..m {
                    let first = previous[j-1];
                    let second = previous[j];
                    result[j] = first + second;
                }
                results.push(result);
            }

        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let num_rows = 5;
        let result = Solution::generate(num_rows);
        assert_eq!(result, vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1], vec![1,4,6,4,1]]);
    }

    #[test]
    fn example_2() {
        let num_rows = 1;
        let result = Solution::generate(num_rows);
        assert_eq!(result, vec![vec![1]]);
    }

}
