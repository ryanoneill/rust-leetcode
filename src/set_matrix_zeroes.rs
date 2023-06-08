/// Given an `m x n` integer matrix `matrix`, if an element is `0`, set its
/// entire row and column to `0`'s.
///
/// You mut do it in place.
struct Solution;

impl Solution {

    fn set_row(matrix: &mut Vec<Vec<i32>>, row: usize) {
        for i in 0..matrix[row].len() {
            matrix[row][i] = 0;
        }
    }

    fn set_col(matrix: &mut Vec<Vec<i32>>, col: usize) {
        for i in 0..matrix.len() {
            matrix[i][col] = 0;
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut first_row = false;
        let mut first_col = false;

        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                let value = matrix[row][col];
                if value == 0 {
                    if row == 0 {
                        first_row = true;
                    } else {
                        matrix[row][0] = 0;
                    }
                    if col == 0 {
                        first_col = true;
                    } else {
                        matrix[0][col] = 0;
                    }
                }
            }
        }

        for row in 1..matrix.len() {
            if matrix[row][0] == 0 {
                Self::set_row(matrix, row);
            }
        }

        for col in 1..matrix[0].len() {
            if matrix[0][col] == 0 {
                Self::set_col(matrix, col);
            }
        }

        if first_row {
            Self::set_row(matrix, 0);
        }

        if first_col {
            Self::set_col(matrix, 0);
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut matrix = vec![
            vec![1,1,1],
            vec![1,0,1],
            vec![1,1,1],
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]]);
    }

    #[test]
    fn example_2() {
        let mut matrix = vec![
            vec![0,1,2,0],
            vec![3,4,5,2],
            vec![1,3,1,5],
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]]);
    }

    #[test]
    fn real_world_1() {
        let mut matrix = vec![
            vec![-1],
            vec![2],
            vec![3]
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![-1], vec![2], vec![3]]);
    }

}
