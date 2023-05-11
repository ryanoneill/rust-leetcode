/// Given a 0-indexed `n x n` integer matrix `grid`, return the number of pairs
/// `(ri, cj)` such that row `ri` and column `cj` are equal.
///
/// A row and column pair is considered equal if they contain the same elements
/// in the same order (i.e., an equal array).
struct Solution;

impl Solution {

    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut result = 0;

        for row in 0..n {
            for col in 0..n {
                let mut are_equal = true;
                for i in 0..n {
                    if grid[row][i] != grid[i][col] {
                        are_equal = false;
                        break;
                    }
                }
                if are_equal {
                    result += 1;
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
        let grid = vec![vec![3,2,1], vec![1,7,6], vec![2,7,7]];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![3,1,2,2], vec![1,4,4,5], vec![2,4,2,2], vec![2,4,2,2]];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn no_equals() {
        let grid = vec![vec![1, 2], vec![3,4]];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn single() {
        let grid = vec![vec![1]];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 1);
    }

}
