/// You are given `row x col` `grid` representing a map where `grid[i][j] = 1` represents land and
/// `grid[i][j] = 0` represents water.
///
/// Grid cells are connected horizontall/vertically (not diagonally). The `grid` is completely
/// surrounded by water, and there is exactly one island (e.g., one or more connected land cells).
///
/// The island doesn't have "lakes", meaning the water inside isn't connected to the water around
/// the island. One cell is a square with side length 1. The grid is rectangular, width and height
/// don't exceed 100. Determine the perimeter of the island.
struct Solution;

impl Solution {

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    // above
                    let above = if i == 0 { 0 } else { grid[i-1][j] };

                    // below
                    let below = if i == m-1 { 0 } else { grid[i+1][j] };

                    // left
                    let left = if j == 0 { 0 } else { grid[i][j-1] };

                    // right
                    let right = if j == n-1 { 0 } else { grid[i][j+1] };

                    let count = above + below + left + right;
                    let sides = 4 - count;
                    result += sides;
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
            vec![0,1,0,0],
            vec![1,1,1,0],
            vec![0,1,0,0],
            vec![1,1,0,0],
        ];
        let result = Solution::island_perimeter(grid);
        assert_eq!(result, 16);
    }

    #[test]
    fn example_2() {
        let grid = vec![
            vec![1],
        ];
        let result = Solution::island_perimeter(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let grid = vec![
            vec![1, 0],
        ];
        let result = Solution::island_perimeter(grid);
        assert_eq!(result, 4);
    }

}
