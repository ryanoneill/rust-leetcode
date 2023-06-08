/// Given an `m x n` matrix `grid` which is sorted in non-increasing order both row-wise and
/// column-wise, return the number of negative numbers in `grid`.
struct Solution;

impl Solution {

    fn count_row_negatives(row: &Vec<i32>) -> i32 {
        let n = row.len();
        let mut start = 0;
        let mut end = n - 1;

        while start <= end {
            let mid = start + (end - start) / 2;
            if row[mid] < 0 {
                if mid == 0 {
                    break;
                } else {
                    end = mid - 1;
                }
            } else {
                start = mid + 1;
            }
        }

        (n - start) as i32
    }

    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let mut result = 0;
        for i in 0..m {
            let value = Self::count_row_negatives(&grid[i]);
            result += value;
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
            vec![4,3,2,-1],
            vec![3,2,1,-1],
            vec![1,1,-1,-2],
            vec![-1,-1,-2,-3]
        ];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![3,2], vec![1,0]];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn real_world_1() {
        let grid = vec![vec![1,-1], vec![-1,-1]];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 3);
    }

}
