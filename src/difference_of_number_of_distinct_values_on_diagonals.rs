use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct DiagonalValues {
    left_side: Vec<i32>,
    current: i32,
    right_side: VecDeque<i32>,
}

impl DiagonalValues {

    pub fn new(current: i32) -> Self {
        Self { left_side: vec![], current, right_side: VecDeque::new() }
    }

    pub fn calc(&self) -> i32 {
        let left_unique: HashSet<i32> = HashSet::from_iter(self.left_side.iter().copied());
        let right_unique: HashSet<i32> = HashSet::from_iter(self.right_side.iter().copied());
        let left_count = left_unique.len() as i32;
        let right_count = right_unique.len() as i32;
        (left_count - right_count).abs()
    }

    pub fn add_right(&mut self, value: i32) {
        self.right_side.push_back(value);
    }

    pub fn shift_right(&mut self) -> bool {
        let result = !self.right_side.is_empty();
        if result {
            self.left_side.push(self.current);
            self.current = self.right_side.pop_front().unwrap();
        }
        result
    }

}

/// Given a 0-indexed 2D `grid` of size `m x n`, you should find the matrix `answer` of size `m x
/// n`.
///
/// The value of each cell `(r, c)` of the matrix `answer` is calculated in the following way:
///
/// * Let `topLeft[r][c]` be the number of distinct values in the top-left diagonal of the cell
/// `(r, c)` in the matrix `grid`.
///
/// * Let `bottomRight[r][c]` be the number of distinct values in the bottom-right diagonal of the
/// cell `(r, c)` in the matrix `grid`.
///
/// Then `answer[r][c] = |topLeft[r][c] - bottomRight[r][c]|`.
///
/// Return the matrix `answer`.
///
/// A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row
/// or leftmost column and going in the bottom-right direction until reaching the matrix's end.
///
/// A cell `(r1, c1)` belongs to the top-left diagonal of the cell `(r, c)`, if both belong to the
/// same diagonal and `r1 < r`. Similarly is defined bottom-right diagonal.
struct Solution;

impl Solution {

    fn fill_in_diagonal(grid: &Vec<Vec<i32>>, result: &mut [Vec<i32>], start: (usize, usize)) {
        let m = grid.len();
        let n = grid[0].len();

        let mut row = start.0;
        let mut col = start.1;

        let mut diagonal_values = DiagonalValues::new(grid[row][col]);
        loop {
            row += 1;
            col += 1;
            if row <= m-1 && col <= n-1 {
                diagonal_values.add_right(grid[row][col]);
            } else {
                break;
            }
        }

        row = start.0;
        col = start.1;
        result[row][col] = diagonal_values.calc();
        loop {
            row += 1;
            col += 1;
            if diagonal_values.shift_right() {
                result[row][col] = diagonal_values.calc();
            } else {
                break;
            }
        }

    }

    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = grid.len();
        let col_len = grid[0].len();

        let mut result = vec![vec![0; col_len]; row_len];
        for i in 0..row_len {
            // Start at column 0 of every row
            Self::fill_in_diagonal(&grid, &mut result, (i, 0));
        }

        for i in 1..col_len {
            // Start at row 0 of every column but the first one (covered above)
            Self::fill_in_diagonal(&grid, &mut result, (0, i));
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![1,2,3], vec![3,1,5], vec![3,2,1]];
        let result = Solution::difference_of_distinct_values(grid);
        assert_eq!(result, vec![vec![1,1,0], vec![1,0,1], vec![0,1,1]]);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![1]];
        let result = Solution::difference_of_distinct_values(grid);
        assert_eq!(result, vec![[0]]);
    }

    #[test]
    fn real_world_1() {
        // 28 -> 38 -> 17 -> 45
        let grid = vec![
            vec![6,28,37,34,12,30,43,35,6],
            vec![21,47,38,14,31,49,11,14,49],
            vec![6,12,35,17,17,2,45,27,43],
            vec![34,41,30,28,45,24,50,20,4]
        ];
        let result = Solution::difference_of_distinct_values(grid);
        assert_eq!(result, vec![
            vec![3,3,3,3,3,3,2,1,0],
            vec![2,1,1,1,1,1,1,0,1],
            vec![1,0,1,1,1,1,1,1,2],
            vec![0,1,2,3,3,3,3,3,3]
        ]);
    }

}
