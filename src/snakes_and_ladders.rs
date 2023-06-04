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

    // TODO: Implement
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
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

    #[ignore]
    #[test]
    fn example_2() {
        let board = vec![
            vec![-1,-1],
            vec![-1,3],
        ];
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, 1);
    }

}
