use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an `m x n` matrix `maze` (0-indexed) with empty cells
/// (represented as '.') and walls (represented as '+'). You are also given the
/// `entrance` of the maze, where `entrance = [entrance_row, entrance_col]
/// denotes the row and column of the cell you are initially standing at.
///
/// In one step, you can move one cell up, down, left, or right. You cannote
/// step into a cell with a wall, and you cannot step outside the maze. Your
/// goal is to find the nearest exit from the `entrance`. An exit is defined
/// as an empty cell that is at the border of the `maze`. The entrance does not
/// count as an exit.
///
/// Return the number of steps in the shortest path from the `entrance` to the
/// nearest exit, or `-1` if no such path exists.
struct State {
    row: usize,
    col: usize,
    steps: i32,
}

impl State {
    fn new(row: usize, col: usize) -> Self {
        State { row, col, steps: 0 }
    }

    fn from_entrance(entrance: &Vec<i32>) -> Self {
        State::new(entrance[0] as usize, entrance[1] as usize)
    }

    fn from_old(row: usize, col: usize, state: &State) -> Self {
        State {
            row,
            col,
            steps: state.steps + 1,
        }
    }

    fn key(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn is_entrance(&self, entrance: &Vec<i32>) -> bool {
        self.row == entrance[0] as usize && self.col == entrance[1] as usize
    }

    fn is_exit(&self, entrance: &Vec<i32>, m: usize, n: usize) -> bool {
        !self.is_entrance(entrance) && (self.is_exit_row(m) || self.is_exit_col(n))
    }

    fn is_exit_col(&self, n: usize) -> bool {
        self.col == 0 || self.col == n - 1
    }

    fn is_exit_row(&self, m: usize) -> bool {
        self.row == 0 || self.row == m - 1
    }
}

struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let m = maze.len();
        let n = maze[0].len();

        let directions = vec!['N', 'S', 'E', 'W'];
        let start = State::from_entrance(&entrance);
        seen.insert(start.key());
        queue.push_back(start);
        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            if state.is_exit(&entrance, m, n) {
                result = state.steps;
                break;
            } else {
                // let's move
                for dir in &directions {
                    let mut row = state.row as i32;
                    let mut col = state.col as i32;
                    match dir {
                        'N' => {
                            row -= 1;
                        }
                        'S' => {
                            row += 1;
                        }
                        'E' => {
                            col += 1;
                        }
                        'W' => {
                            col -= 1;
                        }
                        _ => {}
                    }
                    let valid_row = row >= 0 && row < m as i32;
                    let valid_col = col >= 0 && col < n as i32;
                    if valid_row && valid_col {
                        let row = row as usize;
                        let col = col as usize;
                        if maze[row][col] == '.' {
                            let new_state = State::from_old(row, col, &state);
                            if !seen.contains(&new_state.key()) {
                                seen.insert(new_state.key());
                                queue.push_back(new_state);
                            }
                        }
                    }
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
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let maze = vec![vec!['.', '+']];
        let entrance = vec![0, 0];
        let result = Solution::nearest_exit(maze, entrance);
        assert_eq!(result, -1);
    }
}
