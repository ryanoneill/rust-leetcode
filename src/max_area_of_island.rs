use std::collections::HashSet;

/// You are given an `m x n` binary matrix `grid`. An island is a group of
/// `1`'s (representing land) connected 4-directionally (horizontal or
/// vertical.) You may assume all four edges of the grid are surrounded by
/// water.
///
/// The area of an island is the number of cells with a value `1` in the
/// island.
///
/// Return the maximum area of an island in `grid`. If there is no island,
/// return `0`.
struct Solution;

impl Solution {
    fn row_len(grid: &Vec<Vec<i32>>) -> usize {
        grid.len()
    }

    fn col_len(grid: &Vec<Vec<i32>>) -> usize {
        if Self::row_len(grid) > 0 {
            grid[0].len()
        } else {
            0
        }
    }

    fn is_land(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
        grid[row][col] == 1
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut seen = HashSet::new();

        for row in 0..Self::row_len(&grid) {
            for col in 0..Self::col_len(&grid) {
                let key = (row, col);
                if Self::is_land(&grid, row, col) && !seen.contains(&key) {
                    seen.insert(key);
                    let area = Self::worker(&grid, &mut seen, row, col);
                    if area > result {
                        result = area;
                    }
                }
            }
        }

        result
    }

    fn is_valid_row(grid: &Vec<Vec<i32>>, row: i32) -> bool {
        row >= 0 && row < Self::row_len(&grid) as i32
    }

    fn is_valid_col(grid: &Vec<Vec<i32>>, col: i32) -> bool {
        col >= 0 && col < Self::col_len(&grid) as i32
    }

    fn worker(
        grid: &Vec<Vec<i32>>,
        seen: &mut HashSet<(usize, usize)>,
        row: usize,
        col: usize,
    ) -> i32 {
        let mut result = 1;
        let directions = vec!['N', 'S', 'E', 'W'];

        for dir in &directions {
            let mut neighbor_row = row as i32;
            let mut neighbor_col = col as i32;

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
            let valid_row = Self::is_valid_row(grid, neighbor_row);
            let valid_col = Self::is_valid_col(grid, neighbor_col);
            if valid_row && valid_col {
                let neighbor_row = neighbor_row as usize;
                let neighbor_col = neighbor_col as usize;
                let key = (neighbor_row, neighbor_col);
                if Self::is_land(grid, neighbor_row, neighbor_col) && !seen.contains(&key) {
                    seen.insert(key);
                    result += Self::worker(grid, seen, neighbor_row, neighbor_col);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let result = Solution::max_area_of_island(grid);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        let result = Solution::max_area_of_island(grid);
        assert_eq!(result, 0);
    }
}
