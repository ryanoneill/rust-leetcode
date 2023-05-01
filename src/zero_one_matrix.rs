use std::collections::{VecDeque, HashSet};

/// Given an `m x n` binary matrix `mat`, return the distance of the nearest `0` for
/// each cell.
///
/// The distance between two adjacent cells is `1`.
struct State {
    row: usize,
    col: usize,
    steps: usize
}

struct Solution;

impl Solution {

    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let m = mat.len();
        if m > 0 {
            let n = mat[0].len();
            let directions = vec!['N', 'S', 'E', 'W'];
            let mut queue = VecDeque::new();
            let mut seen = HashSet::new();

            for row in 0..m {
                for col in 0..n {
                    if mat[row][col] == 0 {
                        let state = State { row, col, steps: 1 };
                        queue.push_back(state);
                        seen.insert((row, col));
                    }
                }
            }

            while !queue.is_empty() {
                let state = queue.pop_front().unwrap();
                let steps = state.steps;

                for dir in &directions {
                    let mut row = state.row as i32;
                    let mut col = state.col as i32;

                    match dir {
                        'N' => { row -= 1; }
                        'S' => { row += 1; }
                        'E' => { col += 1; }
                        'W' => { col -= 1; }
                        _ =>  { } // do nothing
                    }
                    let valid_row = row >= 0 && row < m as i32;
                    let valid_col = col >= 0 && col < n as i32;
                    if valid_row && valid_col {
                        let row = row as usize;
                        let col = col as usize;
                        if mat[row][col] == 1 && !seen.contains(&(row, col)) {
                            let state = State { row, col, steps: steps + 1 };
                            seen.insert((row, col));
                            queue.push_back(state);
                            mat[row][col] = steps as i32;
                        }
                    }
                }
            }
        }

        mat
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let mat = vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]];
        let result = Solution::update_matrix(mat);
        assert_eq!(result, vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]]);
    }

    #[test]
    fn example_2() {
        let mat = vec![vec![0,0,0], vec![0,1,0], vec![1,1,1]];
        let result = Solution::update_matrix(mat);
        assert_eq!(result, vec![vec![0,0,0], vec![0,1,0], vec![1,2,1]]);
    }

}
