use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an `m x n` `grid` where each cell can have one of three
/// values:
///
/// * `0` representing an empty cell,
///
/// * `1` representing a fresh orange, or
///
/// * `2` representing a rotten orange.
///
/// Every minute any fresh orange that is 4-directionally adjacent to a rotten
/// orange becomes rotten.
///
/// Return the minimum number of minutes that must elapse until no cell has a
/// fresh orange. If this is impossible, return `-1`.
struct Solution;

impl Solution {

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fresh = HashSet::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                let state = grid[i][j];
                if state == 1 {
                    fresh.insert((i, j));
                } else if state == 2 {
                    seen.insert((i, j));
                    queue.push_back((i, j));
                }
            }
        }

        if fresh.is_empty() { 0 }
        else {
            let directions = vec!['N', 'S', 'E', 'W'];
            let mut result = 0;
            while !queue.is_empty() && !fresh.is_empty() {
                result += 1;
                let q = queue.len();
                for _ in 0..q {
                    let (i, j) = queue.pop_front().unwrap();
                    for dir in &directions {
                        let mut row = i as i32;
                        let mut col = j as i32;
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
                            if !seen.contains(&(row, col)) {
                                if grid[row][col] == 1 {
                                    seen.insert((row, col));
                                    fresh.remove(&(row, col));
                                    queue.push_back((row, col));
                                }
                            }
                        }
                    }
                }
            }

            if fresh.is_empty() { result } else { -1 }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![2,1,1], vec![1,1,0], vec![0,1,1]];
        let result = Solution::oranges_rotting(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![2,1,1], vec![0,1,1], vec![1,0,1]];
        let result = Solution::oranges_rotting(grid);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let grid = vec![vec![0,2]];
        let result = Solution::oranges_rotting(grid);
        assert_eq!(result, 0);
    }

}
