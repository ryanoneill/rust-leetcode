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

    pub fn is_last_row(&self, n: usize) -> bool {
        self.row == n - 1
    }

    pub fn down(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    pub fn can_go_down_left(&self, n: usize) -> bool {
        self.row < n - 1 && self.col > 0
    }

    pub fn down_left(&self) -> Self {
        Self::new(self.row + 1, self.col - 1)
    }

    pub fn can_go_down_right(&self, n: usize) -> bool {
        self.row < n - 1 && self.col < n - 1
    }

    pub fn down_right(&self) -> Self {
        Self::new(self.row + 1, self.col + 1)
    }


}

/// Given an `n x n` array of integers `matrix`, return the minimum sum of any falling path through
/// `matrix`.
///
/// A falling path starts at any element in the first row and chooses either the element in the
/// next row that is either directly below or diagonally left/right. Specifically, the next element
/// from position `(row, col)` will be `(row + 1, col - 1)`, `(row + 1, col)`, or
/// `(row + 1, col + 1)`.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<Position, i32>, matrix: &Vec<Vec<i32>>, pos: Position) -> i32 {
        let n = matrix.len();

        if results.contains_key(&pos) {
            results[&pos]
        } else if pos.is_last_row(n) {
            println!("{:?}", pos);
            matrix[pos.row][pos.col]
        } else {
            let down_position = pos.down();
            let mut result = Self::worker(results, matrix, down_position);

            if pos.can_go_down_left(n) {
                let down_left_position = pos.down_left();
                result = result.min(Self::worker(results, matrix, down_left_position));
            }

            if pos.can_go_down_right(n) {
                let down_right_position = pos.down_right();
                result = result.min(Self::worker(results, matrix, down_right_position));
            }

            result += matrix[pos.row][pos.col];

            results.insert(pos, result);
            result
        }

    }

    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut results = HashMap::new();
        let mut result = i32::MAX;

        for i in 0..n {
            let pos = Position::new(0, i);
            result = result.min(Self::worker(&mut results, &matrix, pos));
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![
            vec![2,1,3],
            vec![6,5,4],
            vec![7,9,9],
        ];
        let result = Solution::min_falling_path_sum(matrix);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let matrix = vec![
            vec![-19,57],
            vec![-40,-5],
        ];
        let result = Solution::min_falling_path_sum(matrix);
        assert_eq!(result, -59);
    }

}
