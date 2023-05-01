use std::collections::HashSet;

/// Given an `m x n` 2D binary `grid` which represents a map of `'1'`s (land)
/// and `'0'`s (water), return the number of islands.
///
/// An island is surrounded by water and is formed by connecting adjacent lands
/// horizontally or vertically. You may assume all four edges of the grid are
/// surrounded by water.
struct Solution;

impl Solution {
    fn row_len(grid: &Vec<Vec<char>>) -> usize {
        grid.len()
    }

    fn col_len(grid: &Vec<Vec<char>>) -> usize {
        if Self::row_len(grid) > 0 {
            grid[0].len()
        } else {
            0
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let mut seen = HashSet::new();

        for row in 0..Self::row_len(&grid) {
            for col in 0..Self::col_len(&grid) {
                let key = (row, col);
                if Self::is_land(&grid, row, col) && !seen.contains(&key) {
                    result += 1;
                    seen.insert(key);
                    Self::worker(&grid, &mut seen, row, col);
                }
            }
        }

        result
    }

    fn is_land(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        grid[row][col] == '1'
    }

    fn worker(grid: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize)>, row: usize, col: usize) {
        let directions = vec!['N', 'S', 'E', 'W'];

        for dir in &directions {
            let mut neighbor_row: i32 = row as i32;
            let mut neighbor_col: i32 = col as i32;

            match dir {
                'N' => {
                    neighbor_row -= 1;
                }
                'S' => {
                    neighbor_row += 1;
                }
                'E' => {
                    neighbor_col += 1;
                }
                'W' => {
                    neighbor_col -= 1;
                }
                _ => {}
            }
            let valid_row = neighbor_row >= 0 && neighbor_row < Self::row_len(&grid) as i32;
            let valid_col = neighbor_col >= 0 && neighbor_col < Self::col_len(&grid) as i32;
            if valid_row && valid_col {
                let neighbor_row = neighbor_row as usize;
                let neighbor_col = neighbor_col as usize;
                let key = (neighbor_row, neighbor_col);
                if Self::is_land(grid, neighbor_row, neighbor_col) && !seen.contains(&key) {
                    seen.insert(key);
                    Self::worker(grid, seen, neighbor_row, neighbor_col);
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
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let result = Solution::num_islands(grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let result = Solution::num_islands(grid);
        assert_eq!(result, 3);
    }
}
