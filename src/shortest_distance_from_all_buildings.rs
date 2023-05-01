use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// You are given an `m x n` grid `grid` of values `0`, `1`, or `2`, where:
///
/// * each `0` marks an empty land that you can pass by freely,
/// * each `1` marks a building that you cannot pass through, and
/// * each `2` marks an obstacle that you cannot pass through.
///
/// You want to build a house on an empty land that reaches all buildings in
/// the shortest total travel distance. You can only move up, down, left, and
/// right.
///
/// Return the shortest travel distance for such a house. If it is not possible
/// to build such a house according to the above rules, return -1.
///
/// The total travel distance is the sum of the distances between the houses of
/// the friends and the meeting point.
///
/// The distance is calculated using Manhattan Distance, where
/// `distance(p1, p2) = |p2.x - p1.x| + |p2.y - p1.y|`.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {

    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: Position,
    steps: i32,
}

impl State {

    fn new(pos: Position) -> Self {
        Self { pos, steps: 0 }
    }

    fn from_old(pos: Position, state: &State) -> Self {
        Self { pos, steps: state.steps + 1 }
    }

    fn key(&self) -> Position {
        self.pos
    }

}

struct Solution;

impl Solution {

    fn is_building(num_type: i32) -> bool {
        num_type == 1
    }

    fn is_empty_land(num_type: i32) -> bool {
        num_type == 0
    }

    fn is_obstacle(num_type: i32) -> bool {
        num_type == 2
    }

    fn find_buildings(grid: &Vec<Vec<i32>>) -> Vec<Position> {
        let mut result = Vec::new();
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if Self::is_building(grid[i][j]) {
                    let position = Position::new(i, j);
                    result.push(position);
                }
            }
        }

        result
    }

    fn calculate_distance(
        grid: &Vec<Vec<i32>>,
        start: Position,
        end: Position
    ) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let directions = vec!['N', 'S', 'E', 'W'];
        let mut result = -1;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        let start_state = State::new(start);
        seen.insert(start_state.key());
        queue.push_back(start_state);
        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            if state.pos == end {
                result = state.steps;
                break;
            } else if Self::is_empty_land(grid[state.pos.row][state.pos.col]) {
                for dir in &directions {
                    let mut row = state.pos.row as i32;
                    let mut col = state.pos.col as i32;

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
                        let new_state = State::from_old(Position::new(row, col), &state);
                        if !Self::is_obstacle(grid[row][col]) && !seen.contains(&new_state.key()) {
                            seen.insert(new_state.key());
                            queue.push_back(new_state);
                        }
                    }
                }
            }
        }

        result
    }

    // TODO: Implement Result Caching
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = -1;
        let buildings = Self::find_buildings(&grid);
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if Self::is_empty_land(grid[i][j]) {
                    let mut current = 0;
                    let pos = Position::new(i, j);
                    for building in &buildings {
                        if current != -1 {
                            let result = Self::calculate_distance(&grid, pos, *building);
                            if result != -1 {
                                current += result;
                            } else {
                                current = -1;
                            }
                        }
                    }
                    if current != -1 {
                        if result == -1 || current < result {
                            result = current;
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
        let grid = vec![vec![1,0,2,0,1], vec![0,0,0,0,0], vec![0,0,1,0,0]];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![1,0]];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let grid = vec![vec![1]];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, -1);
    }

    #[test]
    fn real_world_1() {
        let grid = vec![vec![1,1,1,1,1,0], vec![0,0,0,0,0,1], vec![0,1,1,0,0,1], vec![1,0,0,1,0,1], vec![1,0,1,0,0,1], vec![1,0,0,0,0,1], vec![0,1,1,1,1,0]];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 88);
    }

}
