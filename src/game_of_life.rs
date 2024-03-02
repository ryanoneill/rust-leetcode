/// According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular
/// automaton devised by the British mathematician John Horton Conway in 1970."
///
/// The board is made up of an `m x n` grid of cells, where each cell has an initial state: live
/// (represented by a `1`) or dead (represented by a `0`). Each cell interacts with its eight
/// neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above
/// Wikipedia article):
///
/// 1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
///
/// 2. Any live cell with two or three live neighbors lives on to the next generation.
///
/// 3. Any live cell with more than three live neighbors dies, as if by over-population.
///
/// 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
///
/// The next state is created by applying the above rules simultaneously to every cell in the
/// current state, where births and deaths occur simultaneously. Given the current state of the `m
/// x n` grid `board`, return the next state.
struct Solution;

impl Solution {

    fn count_live_neighbors(board: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        let mut result = 0;

        let rows = board.len();
        let cols = board[0].len();

        let can_go_up = row > 0;
        let can_go_down = row + 1 < rows;
        let can_go_left = col > 0;
        let can_go_right = col + 1 < cols;

        // above row
        if can_go_up {
            if can_go_left {
                result += board[row - 1][col - 1];
            }
            result += board[row - 1][col];
            if can_go_right {
                result += board[row - 1][col + 1];
            }
        }

        // same row (only to the left and to the right)
        if can_go_left {
            result += board[row][col - 1];
        }
        if can_go_right {
            result += board[row][col + 1];
        }
        
        // below row
        if can_go_down {
            if can_go_left {
                result += board[row + 1][col - 1];
            }
            result += board[row + 1][col];
            if can_go_right {
                result += board[row + 1][col + 1];
            }
        }

        result
    }

    // 1 <= m, n <= 25
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();

        // TODO: Don't allocate another entire board. It's not necessary.
        let mut alt = vec![vec![0; n]; m];

        for row in 0..m {
            for col in 0..n {
                let value = board[row][col];
                let live_neighbors = Self::count_live_neighbors(&board, row, col);

                if value == 1 { // live
                    if live_neighbors == 2 || live_neighbors == 3 {
                        alt[row][col] = 1; // still alive
                    } else {
                        alt[row][col] = 0; // now dead
                    }
                } else { // dead
                    if live_neighbors == 3 {
                        alt[row][col] = 1; // now alive
                    } else {
                        alt[row][col] = 0; // still dead
                    }
                }
            } 
        }

        for row in 0..m {
            for col in 0..n {
                board[row][col] = alt[row][col];
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
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0]
        ];
        Solution::game_of_life(&mut board);
        let expected = vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![0, 1, 0]
        ];
        assert_eq!(board, expected);
    }

    #[test]
    fn example_2() {
        let mut board = vec![
            vec![1, 1],
            vec![1, 0]
        ];
        Solution::game_of_life(&mut board);
        let expected = vec![
            vec![1, 1],
            vec![1, 1]
        ];
        assert_eq!(board, expected);
    }

}
