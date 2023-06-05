use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {

    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

}

/// You are a hiker preparing for an upcoming hike. You are given `heights`, a 2D array of size
/// `rows x columns`, where `heights[row][col]` represents the height of cell `(row, col)`. You are
/// situated in the top-left cell, `(0, 0)`, and you hope to travel to the bottom-right cell,
/// `(rows-1, columns-1)` (i.e., 0-indexed). You can move up, down, left, or right, and you wish to
/// find a route that requires the minimum effort.
///
/// A route's effort is the maximum absolute difference in heights between the two consecutive
/// cells of the route.
///
/// Return the minimum effort required to travel from the top-left cell to the bottom-right cell.
struct Solution;

impl Solution {

    fn find_min_max(heights: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut min_result = i32::MAX;
        let mut max_result = i32::MIN;

        for i in 0..heights.len() {
            let row = &heights[i];
            for j in 0..row.len() {
                let height = heights[i][j];
                min_result = min_result.min(height);
                max_result = max_result.max(height);
            }
        }

        (min_result, max_result)
    }

    fn can_finish(heights: &Vec<Vec<i32>>, max_difference: i32) -> bool {
        let mut result = false;
        let mut stack = Vec::new();
        let mut seen = HashSet::new();

        let start = Position::new(0, 0);
        let end = Position::new(heights.len() - 1, heights[0].len() - 1);
        let directions = vec!['N', 'S', 'E', 'W'];

        stack.push(start);
        seen.insert(start);
        while !stack.is_empty() {
            let position = stack.pop().unwrap();
            if position == end {
                result = true;
                break;
            } else {
                for i in 0..directions.len() {
                    let direction = directions[i];
                    let mut row = position.row as i32;
                    let mut col = position.col as i32;
                    match direction {
                        'N' => { row -= 1; }
                        'S' => { row += 1; }
                        'E' => { col += 1; }
                        'W' => { col -= 1; }
                        _   => { }
                    }
                    let valid_row = row >= start.row as i32 && row <= end.row as i32;
                    let valid_col = col >= start.col as i32 && col <= end.col as i32;
                    if valid_row && valid_col {
                        let new_position = Position::new(row as usize, col as usize);
                        if !seen.contains(&new_position) {
                            let new_height = heights[new_position.row][new_position.col];
                            let old_height = heights[position.row][position.col];
                            let diff = (new_height - old_height).abs();
                            if diff <= max_difference {
                                stack.push(new_position);
                                seen.insert(new_position);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (min_height, max_height) = Self::find_min_max(&heights);

        let mut start = 0;
        let mut end = max_height - min_height;

        while start <= end {
            let mid = start + (end - start) / 2;

            if Self::can_finish(&heights, mid) {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        start
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights = vec![
            vec![1,2,2],
            vec![3,8,2],
            vec![5,3,5],
        ];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let heights = vec![
            vec![1,2,3],
            vec![3,8,4],
            vec![5,3,5],
        ];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let heights = vec![
            vec![1,2,1,1,1],
            vec![1,2,1,2,1],
            vec![1,2,1,2,1],
            vec![1,2,1,2,1],
            vec![1,1,1,2,1],
        ];
        let result = Solution::minimum_effort_path(heights);
        assert_eq!(result, 0);
    }

}
