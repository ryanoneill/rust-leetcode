use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an `m x n` integer matrix `grid` where each cell is either
/// `0` (empty) or `1` (obstacle). You can move up, down, left, or right from
/// and to an empty cell in one step.
///
/// Return the minimum number of steps to walk from the upper left corner
/// `(0,0)` to the lower right corner `(m-1,n-1)` given that you can eliminate
/// at most `k` obstacles. If it is not possible to find such walk return `-1`.
struct State {
    row: usize,
    col: usize,
    remaining: i32,
    steps: i32,
}

impl State {

    fn new(row: usize, col: usize, remaining: i32, steps: i32) -> Self {
        Self { row, col, remaining, steps }
    }

    fn key(&self) -> (usize, usize, i32) {
        (self.row, self.col, self.remaining)
    }

    fn has_remaining(&self) -> bool {
        self.remaining > 0
    }

    fn remove_obstacle(&mut self) {
        self.remaining -= 1;
    }

    fn take_step(&mut self) {
        self.steps += 1;
    }

    fn is_end(&self, m: usize, n: usize) -> bool {
        self.row == m-1 && self.col == n-1
    }

}

struct Solution;

impl Solution {

    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let directions = vec!['N', 'S', 'E', 'W'];
        let m = grid.len();
        let n = grid[0].len();

        let mut found_end = false;
        let mut result = 0;
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let start_state = State { row: 0, col: 0, remaining: k, steps: 0 };
        seen.insert(start_state.key());
        queue.push_back(start_state);
        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            if state.is_end(m, n) {
                found_end = true;
                result = state.steps;
                break;
            }

            for dir in &directions {
                let mut row = state.row as i32;
                let mut col = state.col as i32;
                match dir {
                    'N' => { row -= 1; }
                    'S' => { row += 1; }
                    'E' => { col += 1; }
                    'W' => { col -= 1; }
                    _   => { }
                }
                let valid_row = row >= 0 && row < m as i32;
                let valid_col = col >= 0 && col < n as i32;

                if valid_row && valid_col {
                    let row = row as usize;
                    let col = col as usize;
                    let mut state = State::new(row, col, state.remaining, state.steps);
                    state.take_step();
                    if grid[row][col] == 0 {
                        if !seen.contains(&state.key()) {
                            seen.insert(state.key());
                            queue.push_back(state);
                        }
                    } else if state.has_remaining() {
                        state.remove_obstacle();
                        if !seen.contains(&state.key()) {
                            seen.insert(state.key());
                            queue.push_back(state);
                        }
                    }
                }
            }
        }

        if found_end { result } else { -1 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![0,0,0], vec![1,1,0], vec![0,0,0], vec![0,1,1], vec![0,0,0]];
        let k = 1;
        let result = Solution::shortest_path(grid, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![0,1,1], vec![1,1,1], vec![1,0,0]];
        let k = 1;
        let result = Solution::shortest_path(grid, k);
        assert_eq!(result, -1);
    }

}
