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
struct Solution;

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

    fn calculate_distances(grid: &Vec<Vec<i32>>, start: Position) -> HashMap<Position, i32> {
        let mut results: HashMap<Position, i32> = HashMap::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        let mut steps = 0;

        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let directions = vec!['N', 'S', 'E', 'W'];
        seen.insert(start);
        queue.push_back(start);
        while !queue.is_empty() {
            let q_len = queue.len();
            steps += 1;
            for _ in 0..q_len {
                let pos = queue.pop_front().unwrap();
                for dir in &directions {
                    let mut row = pos.row as i32;
                    let mut col = pos.col as i32;

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
                        let pos = Position::new(row as usize, col as usize);
                        if !seen.contains(&pos) && Self::is_empty_land(grid[pos.row][pos.col]) {
                            seen.insert(pos);
                            queue.push_back(pos);
                            results.insert(pos, steps);
                        }
                    }


                }

            }
        }

        results
    }

    fn calculate_building_distances(grid: &Vec<Vec<i32>>) -> Vec<HashMap<Position, i32>> {
        let mut results = Vec::new();

        let buildings = Self::find_buildings(&grid);
        for building in buildings {
            let part = Self::calculate_distances(grid, building);
            results.push(part);
        }

        results
    }

    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = -1;

        let distances = Self::calculate_building_distances(&grid);

        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if Self::is_empty_land(grid[i][j]) {
                    let pos = Position::new(i, j);
                    let mut total = 0;
                    for dist in &distances {
                        if dist.contains_key(&pos) {
                            total += dist[&pos];
                        } else {
                            total = -1;
                            break;
                        }
                    }
                    if total != -1 {
                        if result == -1 {
                            result = total;
                        } else {
                            result = result.min(total);
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
        let grid = vec![
            vec![1, 0, 2, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![1, 0]];
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
        let grid = vec![
            vec![1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 1, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1],
            vec![0, 1, 1, 1, 1, 0],
        ];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 88);
    }

    // Before Change: Finished in 713.24s
    // After Change: Finished in 3.924s
    #[test]
    fn real_world_2() {
        let grid = vec![vec![0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,2,1,0,0,2,0,0,0,0,0,1,2,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,1,0,0,2,1,2,0,0],vec![0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,2,1,0,2,0,0,2,0,0,0,1,1,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,2,2,0,0,1,2,1,0,0,0,0,0,2,0,2,1,0,0,0,0,0,0,0],vec![0,1,0,0,1,0,0,0,0,0,0,0,2,0,0,2,0,0,0,0,0,0,1,0,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,1,0,0,1,0,0,0,2,0,2,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,2,1,0],vec![2,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0],vec![1,0,2,0,0,1,0,0,2,0,0,1,0,0,0,1,0,0,0,2,0,2,0,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,1,0,0,2,0,0],vec![0,2,0,1,0,0,0,1,0,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0],vec![0,0,0,0,0,0,0,2,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,2,0,0,0,0,0,1,0,0,0,0,0,0,0,0,2,0,1,0,0,0,0,0,0,0,0,0],vec![2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,2,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,0],vec![0,0,0,2,0,0,0,2,0,1,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,2,0,2,0,0,1,0,0,0,1,2,0,0,0,0,0,0,1],vec![1,0,0,0,0,1,1,2,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,1,0,0,0,0,0,2,1,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,2,2,0,0,0,0,2,0,1,1,0,0],vec![0,2,0,0,0,1,1,0,0,0,2,0,0,0,0,0,0,0,0,1,0,1,0,0,0,2,1,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0],vec![0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,2,2,0,1,0,0,0,0,0,0,0,0,1,0,0,0,2,0,0,1,2,0,0,0,2,0,0,1],vec![0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,2,0,0,0,0,1,1,0,0,1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2],vec![0,2,0,2,0,2,0,0,0,0,0,0,0,0,1,2,0,0,0,0,0,0,1,0,1,0,0,2,0,0,1,0,1,0,0,0,0,0,0,0,1,0,2,0,0,0,0,0,0,1],vec![0,0,0,0,1,0,0,1,0,0,1,2,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,2,0,2,0,0,0,1,0,2,0,0,0,0,0,1,2,2,0,0,0,0],vec![0,0,2,0,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,2],vec![0,0,1,0,0,0,1,0,2,0,0,0,0,0,0,0,0,0,0,2,0,1,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0],vec![0,0,0,0,1,2,0,0,0,0,0,1,2,0,0,0,2,0,1,0,0,2,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0],vec![0,2,0,1,0,0,0,2,2,2,0,0,0,0,0,0,0,0,1,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0],vec![0,0,0,2,0,1,0,0,0,0,0,2,0,1,2,0,0,0,0,0,0,2,0,0,0,0,2,1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0],vec![2,0,0,0,2,1,2,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1,0,0,0,0,0,2,2,2,0,0,0,0,0,0,0,1,0,0,0,0,0],vec![0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,2,0,2,0,2,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,1],vec![0,2,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,2,2,0,1,2,0,0,0,0,0,1,0,0,0,0,0,0,0,2,0,0],vec![0,0,2,0,0,1,1,0,0,0,0,0,0,0,0,0,0,2,0,1,0,0,2,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,2],vec![1,0,0,2,1,0,0,0,0,0,0,0,2,1,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,0,1,0,1,0,0,0,0,0,2],vec![2,0,0,0,1,0,0,0,2,0,0,0,2,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,2,0,0,1,0,0,0,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,1,0,1,0,0,2,0,1,0,0,1,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,2,1,0,2,0,0,0,0,0,1,2,0,0,1,0],vec![1,0,0,0,0,0,0,0,2,0,2,0,0,0,1,0,1,0,0,0,0,0,1,0,1,0,0,2,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2,2,0,0,0,0],vec![0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,1,0,0,2,0,0,0,0,0,0,0,0,0,0,2,0,0,2,0,0,0,2,0,0,0,1,0,0,2,0,1,2],vec![0,2,0,0,0,0,0,0,0,0,1,0,0,0,0,0,2,0,1,0,0,0,0,0,2,0,0,0,1,0,1,0,0,0,0,0,0,0,1,0,1,0,0,0,0,2,1,0,0,0],vec![0,0,0,0,0,0,0,0,2,0,0,0,0,0,1,0,0,0,0,0,0,2,1,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0],vec![0,1,0,0,0,0,0,0,2,1,1,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,2,2,0,0,0,0,0,1,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2,1,1,0,0,1,0,0,0,0,0,0,0,0,2,0,2,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0],vec![0,0,1,0,1,0,0,0,2,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,2,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0],vec![0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,2,0,0,0,0,0,1,0,0],vec![0,0,0,0,2,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,1,0,1,2,0,0,0,0,0,0,0,0,0,0,0,1],vec![0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,1,2,0,1,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,2,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,2,0,0,1,0,0,0,0,0,2,0,0,1,0,0,0,0,0,0,0,0,0,0],vec![0,0,1,0,0,0,2,2,0,2,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],vec![0,0,2,0,0,0,0,0,0,0,0,0,0,2,2,0,0,0,2,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0],vec![0,0,0,0,0,1,0,0,0,0,0,1,1,2,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,2,0,0,2,0,0,0,2,0,0,0,0,0,2,0,2,0,0],vec![0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,2,0,0,1,0,0,2,2,0,0,2,0,0,0,1,1,0,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,1,0,0,1,2,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,2,0,0,0,0,0,0,1,0,0,2,0],vec![0,0,0,0,0,2,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0,2,0,0,0,0,0,0,0,0,0,0,0,1,2,0,0,0,0,0,2,0,0,0,0,0,0,0],vec![0,0,0,0,2,0,0,0,1,1,0,0,0,0,0,0,2,0,0,2,0,0,0,1,1,0,0,0,0,2,0,0,2,0,2,0,2,0,0,0,0,0,0,0,0,0,0,1,0,0],vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,2,0,0,2,0,1]];
        let result = Solution::shortest_distance(grid);
        assert_eq!(result, 6321);
    }

}
