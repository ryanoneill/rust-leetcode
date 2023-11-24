/// Given an `m x n` matrix `mat`, return an array of all the elements of the array in a diagonal
/// order.
struct Solution;

impl Solution {

    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let size = m * n;
        let mut result = vec![0; size];

        let mut row = 0;
        let mut col = 0;
        let mut index = 0;
        let mut direction = true;

        while index < size {
            result[index] = mat[row][col];

            if direction {
                if row == 0 || col == n - 1 {
                    direction = false;
                }

                if direction {
                    row -= 1;
                    col += 1;
                } else if col == n - 1 {
                    row += 1;
                } else {
                    col += 1;
                }
            } else {
                if row == m - 1 || col == 0 {
                    direction = true;
                }

                if !direction {
                    row += 1;
                    col -= 1;
                } else if row == m - 1 {
                    col += 1;
                } else {
                    row += 1;
                }
            }

            index += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        let result = Solution::find_diagonal_order(mat);
        assert_eq!(result, vec![1,2,4,7,5,3,6,8,9]);
    }

    #[test]
    fn example_2() {
        let mat = vec![vec![1,2], vec![3,4,]];
        let result = Solution::find_diagonal_order(mat);
        assert_eq!(result, vec![1,2,3,4]);
    }

}
