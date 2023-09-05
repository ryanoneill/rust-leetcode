struct Solution;

impl Solution {

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols+1]; rows+1];

        let mut result = 0;

        for i in 1..=rows {
            for j in 1..=cols {
                let value = matrix[i-1][j-1];
                if value == '1' {
                    let left = dp[i-1][j];
                    let up = dp[i][j-1];
                    let up_left = dp[i-1][j-1];
                    let calculated = left.min(up).min(up_left) + 1;

                    dp[i][j] = calculated;
                    result = result.max(calculated);
                }
            }
        }

        result * result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0'],
        ];
        let result = Solution::maximal_square(matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let matrix = vec![
            vec!['0','1'],
            vec!['1','0'],
        ];
        let result = Solution::maximal_square(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let matrix = vec![vec!['0']];
        let result = Solution::maximal_square(matrix);
        assert_eq!(result, 0);
    }

}
