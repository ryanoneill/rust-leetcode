use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Cell {
    value: i32,
    row: usize,
    col: usize,
}

impl Cell {
    
    pub fn new(value: i32, row: usize, col: usize) -> Self {
        Self { value, row, col }
    }

}

/// Given an `m x n` integers `matrix`, return the length of the longest increasing path in
/// `matrix`.
///
/// From each cell, you can either move in four directions: left, right, up, or down. You may not
/// move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<Cell, i32>, matrix: &Vec<Vec<i32>>, cell: Cell) -> i32 {
        if results.contains_key(&cell) {
            results[&cell]
        } else {
            let value = cell.value;
            let m = matrix.len() as i32;
            let n = matrix[0].len() as i32;

            let mut result = 1;

            let directions = vec!['N', 'S', 'E', 'W'];
            for dir in &directions {
                let mut row = cell.row as i32;
                let mut col = cell.col as i32;

                match dir {
                    'N' => { row -= 1; }
                    'S' => { row += 1; }
                    'E' => { col += 1; }
                    'W' => { col -= 1; }
                    _   => { }
                }

                let valid_row = row >= 0 && row < m;
                let valid_col = col >= 0 && col < n;

                if valid_row && valid_col {
                    let next_row = row as usize;
                    let next_col = col as usize;
                    let next = matrix[next_row][next_col];

                    if next > value {
                        let next_cell = Cell::new(next, next_row, next_col);
                        let next_try = 1 + Self::worker(results, matrix, next_cell);
                        result = result.max(next_try);
                    }
                }
            }

            results.insert(cell, result);
            result
        }
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let size = m * n;

        let mut cells = Vec::with_capacity(size);
        for i in 0..m {
            for j in 0..n {
                let cell = Cell::new(matrix[i][j], i, j);
                cells.push(cell);
            }
        }
        cells.sort_unstable_by(|a, b| b.cmp(a));

        let mut results = HashMap::new();
        let mut result = 0;
        for i in 0..size {
            let cell = cells[i];
            let value = Self::worker(&mut results, &matrix, cell);
            result = result.max(value);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![vec![9,9,4], vec![6,6,8], vec![2,1,1]];
        let result = Solution::longest_increasing_path(matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![3,4,5], vec![3,2,6], vec![2,2,1]];
        let result = Solution::longest_increasing_path(matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let matrix = vec![vec![1]];
        let result = Solution::longest_increasing_path(matrix);
        assert_eq!(result, 1);
    }

}
