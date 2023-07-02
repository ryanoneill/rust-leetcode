use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize
}

impl Position {

    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn go_down(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    fn go_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    fn is_valid(&self, m: usize, n: usize) -> bool {
        self.row < m && self.col < n
    }

}

/// There is a robot on an `m x n` grid. The robot is initially located at the top-left corner
/// (i.e., `grid[0][0]`). The robot tries to move to the bottom-right corner (i.e.
/// `grid[m-1][n-1]`). The robot can only move either down or right at any point in time.
///
/// Given the two integers `m` and `n`, return the number of possible unique paths that the robot
/// can take to reach the bottom-right corner.
///
/// The test cases are generated so that the answer will be less than or equal to `2 * 10^9`.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<Position, i32>, m: usize, n: usize, pos: Position) -> i32 {
        if results.contains_key(&pos) {
            results[&pos]
        } else {
            let mut result = 0;
            let down = pos.go_down();
            if down.is_valid(m, n) {
                result += Self::worker(results, m, n, down);
            }
            let right = pos.go_right();
            if right.is_valid(m, n) {
                result += Self::worker(results, m, n, right);
            }
            results.insert(pos, result);
            result
        }
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut results = HashMap::new();
        let end = Position::new(m - 1, n - 1);
        results.insert(end, 1);

        let start = Position::new(0, 0);
        Self::worker(&mut results, m, n, start)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let m = 3;
        let n = 7;
        let result = Solution::unique_paths(m, n);
        assert_eq!(result, 28);
    }

    #[test]
    fn example_2() {
        let m = 3;
        let n = 2;
        let result = Solution::unique_paths(m, n);
        assert_eq!(result, 3);
    }

}
