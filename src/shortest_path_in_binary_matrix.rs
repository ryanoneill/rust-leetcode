use std::collections::HashSet;
use std::collections::VecDeque;

/// Given an `n x n` binary matrix `grid`, return the length of the shortest
/// clear path in the matrix. If there is no clear path, return `-1`.
///
/// A clear path in a binary matrix is a path from the top-left cell (i.e.,
/// `(0,0)`) to the bottom-right cell (i.e., `(n-1, n-1)`) such that:
///
/// * All the visited cells of the path are `0`.
/// * All the adjacent cells of the path are 8-directionally connected (i.e.,
///   they are different and they share an edge or a corner).
///
/// The length of a clear path is the number of visited cells of this path.
struct State {
    row: usize,
    col: usize,
    steps: i32,
}

impl State {
    pub fn new(row: usize, col: usize, steps: i32) -> Self {
        Self { row, col, steps }
    }

    pub fn key(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn is_end(&self, n: usize) -> bool {
        self.row == n - 1 && self.col == n - 1
    }
}

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut result = 0;
        let mut seen = HashSet::new();
        let mut reached = false;
        let mut queue = VecDeque::new();

        let directions = vec!["N", "S", "E", "W", "NE", "SE", "SW", "NW"];

        queue.push_back(State::new(0, 0, 1));
        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            if grid[state.row][state.col] == 0 && !seen.contains(&state.key()) {
                seen.insert(state.key());
                if state.is_end(n) {
                    reached = true;
                    result = state.steps;
                    break;
                } else {
                    let row = state.row as i32;
                    let col = state.col as i32;

                    for dir in &directions {
                        let mut neighbor_row = row;
                        let mut neighbor_col = col;

                        match *dir {
                            "N" => {
                                neighbor_row -= 1;
                            }
                            "S" => {
                                neighbor_row += 1;
                            }
                            "E" => {
                                neighbor_col += 1;
                            }
                            "W" => {
                                neighbor_col -= 1;
                            }
                            "NW" => {
                                neighbor_row -= 1;
                                neighbor_col -= 1;
                            }
                            "NE" => {
                                neighbor_row -= 1;
                                neighbor_col += 1;
                            }
                            "SW" => {
                                neighbor_row += 1;
                                neighbor_col -= 1;
                            }
                            "SE" => {
                                neighbor_row += 1;
                                neighbor_col += 1;
                            }
                            _ => {}
                        }

                        let valid_row = neighbor_row >= 0 && neighbor_row < (n as i32);
                        let valid_col = neighbor_col >= 0 && neighbor_col < (n as i32);

                        if valid_row && valid_col {
                            let new_state = State::new(
                                neighbor_row as usize,
                                neighbor_col as usize,
                                state.steps + 1,
                            );
                            queue.push_back(new_state);
                        }
                    }
                }
            }
        }

        if reached {
            result
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(result, -1);
    }
}
