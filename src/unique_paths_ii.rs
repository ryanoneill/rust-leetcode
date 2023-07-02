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

    fn is_valid(&self, grid: &Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        self.row < m && self.col < n && grid[self.row][self.col] == 0
    }

}

/// You are given `m x n` integer array `grid`. There is a robot initially located at the top-left
/// corner (i.e. `grid[0][0]`). The robot tries to move to the bottom-right corner (i.e.
/// `grid[m-1][n-1]`). The robot can only move either down or right at any point in time.
///
/// An obstacle and space are marked as `1` or `0` respectively in `grid`. A path that the robot
/// takes cannot include any square that is an obstacle. 
///
/// Return the number of possible unique paths that the robot can take to reach the bottom-right
/// corner.
struct Solution;

impl Solution {

    fn worker(grid: &Vec<Vec<i32>>, results: &mut HashMap<Position, i32>, pos: Position) -> i32 {
        if results.contains_key(&pos) {
            results[&pos]
        } else {
            let mut result = 0;
            let down = pos.go_down();
            if down.is_valid(grid) {
                result += Self::worker(grid, results, down);
            }
            let right = pos.go_right();
            if right.is_valid(grid) {
                result += Self::worker(grid, results, right);
            }
            results.insert(pos, result);
            result
        }
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let end = Position::new(m - 1, n - 1);
        let mut results = HashMap::new();
        results.insert(end, 1);

        let start = Position::new(0, 0);
        if start.is_valid(&obstacle_grid) {
            Self::worker(&obstacle_grid, &mut results, start)
        } else {
            0
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let obstacle_grid = vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]];
        let result = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let obstacle_grid = vec![vec![0,1], vec![0,0]];
        let result = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(result, 1);
    }

}
