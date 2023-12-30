use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn initial() -> Self {
        Self::new(0, 0)
    }

    pub fn down(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    pub fn right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    pub fn is_valid(&self, m: usize, n: usize) -> bool {
        self.row < m && self.col < n
    }

}

/// Given an `m x n` `grid` filled with non-negative numbers, find a path from top left to bottom
/// right, which minimizes the sum of all numbers along its path.
///
/// Note: You can only move either down or right at any point in time.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<Position, i32>, grid: &Vec<Vec<i32>>, pos: Position) -> i32 {
        if results.contains_key(&pos) {
            results[&pos]
        } else {
            let mut result;

            let m = grid.len();
            let n = grid[0].len();
            let end = Position::new(m-1, n-1);

            if pos == end {
                result = grid[end.row][end.col];
            } else {
                let down_pos = pos.down();
                let mut down_result = i32::MAX;
                if down_pos.is_valid(m, n) {
                    down_result = Self::worker(results, grid, down_pos);
                }

                let right_pos = pos.right();
                let mut right_result = i32::MAX;
                if right_pos.is_valid(m, n) {
                    right_result = Self::worker(results, grid, right_pos);
                }

                result = down_result.min(right_result);
                if result != i32::MAX {
                    result += grid[pos.row][pos.col];
                }
            }

            results.insert(pos, result);
            result
        }
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut results = HashMap::new();
        let initial_position = Position::initial();
        let result = Self::worker(&mut results, &grid, initial_position);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![
            vec![1,3,1],
            vec![1,5,1],
            vec![4,2,1],
        ];
        let result = Solution::min_path_sum(grid);
        assert_eq!(result, 7);
    }


    #[test]
    fn example_2() {
        let grid = vec![
            vec![1,2,3],
            vec![4,5,6],
        ];
        let result = Solution::min_path_sum(grid);
        assert_eq!(result, 12);
    }

}
