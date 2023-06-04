use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an `n x n` integer matrix `board` where the cells are labeled
/// from `1` to `n^2` in a Boustrophedon style starting from the bottom left of
/// the board (i.e. `board[n-1][0]`) and alternating direction each row.
///
/// You start on square `1` of the board. In each move, starting from square
/// `curr`, do the following:
///
/// * Choose a desination square `next` with a label in the range
///   `[curr + 1, min(curr + 6, n^2)]`.
///   * This choice simulates the result of a standard 6-sided die roll: i.e.,
///     there are always at most 6 destinations, regardless of the size of the
///     board.
///
///   * If `next` has a snake or ladder, you must move to the destination of
///     that snake or ladder. Otherwise, you move to `next`.
///
///   * The game ends when you reach the square `n^2`.
///
/// A board square on row `r` and column `c` has a snake or ladder if
/// `board[r][c] != -1`. The destination of that snake or ladder is
/// `board[r][c]`. Squares `1` and `n^2` do not have a snake or ladder.
///
/// Note that you only take a snake or ladder at most once per move. If the
/// destination to a snake or ladder is the start of another snake or ladder,
/// you do not follow the subsequent snake or ladder.
///
/// * For example, suppose the board is `[[-1,4], [-1,3]]`, and on the first
///   move, your destination is square `2`. You follow the ladder to square `3`,
///   but do not follow the subsequent ladder to `4`.
///
/// Return the least number of moves required to reach the square `n^2`. If it
/// is not possible to reach the square, return `-1`.
struct Solution;

impl Solution {

    fn square_to_row_column(n: usize, square: usize) -> (usize, usize) {
        // n = 6
        //      0  1  2  3  4  5
        // ---------------------
        // 0|  36 35 34 33 32 31
        // 1|  25 26 27 28 29 30
        // 2|  24 23 22 21 20 19
        // 3|  13 14 15 16 17 18
        // 4|  12 11 10  9  8  7
        // 5|   1  2  3  4  5  6
        let div = (square - 1) / n;
        let row = n - div - 1;
        let rem = (square - 1) % n;
        let col = if div % 2 == 0 { rem } else { n - rem - 1 };

        (row, col)
    }

    fn square_status(board: &Vec<Vec<i32>>, square: usize) -> i32 {
        let n = board.len();
        let (row, col) = Self::square_to_row_column(n, square);
        board[row][col]
    }

    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let end_square = n * n;

        let mut result = 0;
        let mut finished = false;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(1);
        seen.insert(1);
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let square = queue.pop_front().unwrap();
                if square == end_square {
                    finished = true;
                    break;
                } else {
                    for i in 1..=6 {
                        let new_square = square + i;
                        if new_square <= end_square && !seen.contains(&new_square) {
                            seen.insert(new_square);
                            let status = Self::square_status(&board, new_square);
                            if status == -1 {
                                queue.push_back(new_square);
                            } else {
                                queue.push_back(status as usize);
                            }
                        }
                    }
                }
            }
            if finished {
                break;
            } else {
                result += 1;
            }
        }

        if finished { result } else { -1 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let board = vec![
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,35,-1,-1,13,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,15,-1,-1,-1,-1],
        ];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec![-1,-1],
            vec![-1,3],
        ];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 1);
    }

    #[test]
    fn real_world() {
        let board = vec![
            vec![-1,-1,19,10,-1],
            vec![2,-1,-1,6,-1],
            vec![-1,17,-1,19,-1],
            vec![25,-1,20,-1,-1],
            vec![-1,-1,-1,-1,15],
        ];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 2);
    }

}
