/// You are given an `m x n` matrix `board` containing letters `X` and `O`, capture regions that
/// are surrounded:
///
/// * Connect: A cell is connected to adjacent cells horizontally or vertically.
///
/// * Region: To form a region connect every `'O'` cell.
///
/// * Surround: The region is surrounded with `'X'` cells if you can connect the region with `'X'`
/// cells and none of the region cells are on the edge of the board.
///
/// A surrounded region is captured by replacing all `'O'`s with `'X'`s in the input matrix
/// `board`.
struct Solution;

impl Solution {

    fn capture(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let m = board.len();
        let n = board[0].len();

        if board[i][j] == 'O' {
            board[i][j] = 'C';
            if i > 0 {
                Self::capture(board, i - 1, j);
            }
            if j > 0 {
                Self::capture(board, i, j - 1);
            }
            if i < m-1 {
                Self::capture(board, i + 1, j);
            }
            if j < n-1 {
                Self::capture(board, i, j + 1);
            }
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    if i == 0 || i == m-1 {
                        Self::capture(board, i, j);
                    } else if j == 0 || j == n-1 {
                        Self::capture(board, i, j);
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                let value = board[i][j];
                if value == 'O' {
                    board[i][j] = 'X';
                } else if value == 'C' {
                    board[i][j] = 'O';
                }
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]);
    }

    #[test]
    fn example_2() {
        let mut board = vec![
            vec!['X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec![
            vec!['X'],
        ])
    }

    #[test]
    fn example_3() {
        let mut board = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O'],
            vec!['X', 'X', 'O', 'X', 'O'],
        ];
        Solution::solve(&mut board);
        assert_eq!(board, vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'X', 'X', 'X', 'O'],
            vec!['X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O'],
            vec!['X', 'X', 'O', 'X', 'O'],
        ]);
    }

}
