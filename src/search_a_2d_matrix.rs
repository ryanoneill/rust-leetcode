/// You are given an `m x n` integer matrix `matrix` with the following two
/// properties:
///
/// * Each row is sorted in non-decreasing order.
///
/// * The first integer of each row is greater than the last integer of the
///   previous row.
///
/// Given an integer `target`, return `true` if `target` is in `matrix` or
/// `false` otherwise.
///
/// You must write a solution in `O(log(m * n))` time complexity.
struct Solution;

impl Solution {

    fn find_col(matrix: &Vec<Vec<i32>>, target: i32, row: usize) -> usize {
        let n = matrix[0].len();
        Self::find_col_worker(matrix, target, row, 0, n-1)
    }

    fn find_row(matrix: &Vec<Vec<i32>>, target: i32) -> usize {
        let m = matrix.len();
        Self::find_row_worker(matrix, target, 0, m-1)
    }

    fn find_col_worker(matrix: &Vec<Vec<i32>>, target: i32, row: usize, start: usize, end: usize) -> usize {
        let mid = start + (end - start) / 2;
        let value = matrix[row][mid];
        if value == target { 
            mid
        } else if target < value {
            if mid == start {
                start
            } else {
                Self::find_col_worker(matrix, target, row, start, mid - 1)
            }
        } else {
            if mid == end {
                end
            } else {
                Self::find_col_worker(matrix, target, row, mid + 1, end)
            }
        }
    }

    fn find_row_worker(matrix: &Vec<Vec<i32>>, target: i32, start: usize, end: usize) -> usize {
        let mid = start + (end - start) / 2;
        let value = matrix[mid][0];
        if value == target { 
            mid 
        } else if target < value {
            if mid == start {
                if start == 0 {
                    0
                } else {
                    start - 1
                }
            } else {
                Self::find_row_worker(matrix, target, start, mid - 1)
            }
        } else {
            if mid == end {
                end
            } else {
                Self::find_row_worker(matrix, target, mid + 1, end)
            }
        }
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row = Self::find_row(&matrix, target);
        let col = Self::find_col(&matrix, target, row);
        matrix[row][col] == target
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![vec![1,3,5,7], vec![10,11,16,20], vec![23,30,34,60]];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![1,3,5,7], vec![10,11,16,20], vec![23,30,34,60]];
        let target = 13;
        let result = Solution::search_matrix(matrix, target);
        assert!(!result);
    }

}
