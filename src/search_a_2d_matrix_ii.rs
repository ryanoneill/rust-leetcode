/// Write an efficient algorithm that searches for a value `target` in an
/// `m x n` integer matrix `matrix`. This matrix has the following properties:
///
/// * Integers in each row are sorted in ascending from left to right.
///
/// * Integers in each column are sorted in ascending from top to bottom.
struct Solution;

impl Solution {

    fn search_section(matrix: &Vec<Vec<i32>>, target: i32, top: usize, left: usize, bottom: usize, right: usize) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        if left > right || top > bottom {
            false
        } else if bottom > m-1 {
            false
        } else if right > n-1 {
            false
        } else if target < matrix[top][left] {
            false
        } else if target > matrix[bottom][right] {
            false
        } else {
            let mut result = false;
            let mid = left + (right - left) / 2;

            let mut row = top;
            while row <= bottom && matrix[row][mid] <= target {
                if matrix[row][mid] == target {
                    result = true;
                    break;
                } else {
                    row += 1;
                }
            }

            if mid != 0 {
                result = result || Self::search_section(matrix, target, row, left, bottom, mid-1);
            }
            if row != 0 {
                result = result || Self::search_section(matrix, target, top, mid+1, row-1, right);
            }
            result
        }
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        Self::search_section(&matrix, target, 0, 0, m-1, n-1)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![
            vec![1,4,7,11,15],
            vec![2,5,8,12,19],
            vec![3,6,9,16,22],
            vec![10,13,14,17,24],
            vec![18,21,23,26,30]];
        let target = 5;
        let result = Solution::search_matrix(matrix, target);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let matrix = vec![
            vec![1,4,7,11,15],
            vec![2,5,8,12,19],
            vec![3,6,9,16,22],
            vec![10,13,14,17,24],
            vec![18,21,23,26,30]];
        let target = 20;
        let result = Solution::search_matrix(matrix, target);
        assert!(!result);
    }

    #[test]
    fn real_world_1() {
        let matrix = vec![vec![-1,3]];
        let target = 1;
        let result = Solution::search_matrix(matrix, target);
        assert!(!result);
    }

    #[test]
    fn real_world_2() {
        let matrix = vec![vec![-1,3]];
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        assert!(result);
    }

}
