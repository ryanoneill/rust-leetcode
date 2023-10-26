use std::collections::HashSet;

/// The n-queens puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no
/// two queens attack each other.
///
/// Given an integer `n`, return all distinct solutions to the n-queens puzzle. You may return the
/// answer in any order.
///
/// Each solution contains a distinct board configuration of the n-queens' placement, where `Q` and
/// `.` both indicate a queen and an empty space, respectively.
struct Solution;

impl Solution {

    fn backtrack(
        results: &mut Vec<Vec<String>>,
        n: i32,
        row: i32,
        diagonals: HashSet<i32>,
        anti_diagonals: HashSet<i32>,
        columns: HashSet<i32>,
        board: Vec<i32>
    ) {
        if row == n {
            let output = Self::board_to_output(board);
            results.push(output);
        } else {
            for col in 0..n {
                let diagonal = row - col;
                let anti_diagonal = row + col;

                let good = !columns.contains(&col)
                    && !diagonals.contains(&diagonal)
                    && !anti_diagonals.contains(&anti_diagonal);

                if good {
                    let mut columns = columns.clone();
                    let mut diagonals = diagonals.clone();
                    let mut anti_diagonals = anti_diagonals.clone();

                    columns.insert(col);
                    diagonals.insert(diagonal);
                    anti_diagonals.insert(anti_diagonal);

                    let mut board = board.clone();
                    let index = row as usize;
                    board[index] = col;

                    Self::backtrack(results, n, row + 1, diagonals, anti_diagonals, columns, board)
                }
            }
        }
    }

    fn board_to_output(board: Vec<i32>) -> Vec<String> {
        let n = board.len();
        let mut results = Vec::with_capacity(n);

        for i in 0..n {
            let mut result = String::from("");
            let value = board[i] as usize;

            for j in 0..n {
                if j == value {
                    result.push('Q');
                } else {
                    result.push('.');
                }
            }
            results.push(result);
        }

        results
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();

        let empty_board = vec![-1; n as usize];
        Self::backtrack(&mut results, n, 0, HashSet::new(), HashSet::new(), HashSet::new(), empty_board);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let result = Solution::solve_n_queens(n);
        assert_eq!(result, vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."]]);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::solve_n_queens(n);
        assert_eq!(result, vec![vec!["Q"]]);
    }

    #[test]
    fn example_3() {
        let n = 2;
        let result = Solution::solve_n_queens(n);
        let expected: Vec<Vec<String>> = Vec::new();
        assert_eq!(result, expected);
    }

}
