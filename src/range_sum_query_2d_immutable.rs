/// Given a 2D matrix `matrix`, handle multiple queries of the following type:
///
/// * Calculate the sum of the elements of `matrix` inside the rectangle defined by its upper left
///   corner `(row1, col1)` and lower right corner `(row2, col2)`.
///
/// Implement the `NumMatrix` class:
///
/// * `NumMatrix(int[][] matrix)` Initializes the object with the integer matrix `matrix`.
///
/// * `int sumRegion(int row1, int col1, int row2, int col2)` Returns the sum of the elements of
/// `matrix` inside the rectangle defined by its upper left corner `(row1, col1)` and lower right
/// corner `(row2, col2)`.
///
/// You must design an algorithm where `sumRegion` works on `O(1)` time complexity.
struct NumMatrix {
    sums: Vec<Vec<i32>>
}

impl NumMatrix {

    fn make_row_sums(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let mut row_sums = Vec::with_capacity(m);
        for row in matrix {
            let n = row.len();
            let mut row_sum = Vec::with_capacity(n);
            let mut sum = 0;
            for num in row {
                sum += num;
                row_sum.push(sum);
            }
            row_sums.push(row_sum);
        }
        row_sums
    }

    fn make_sums(row_sums: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = row_sums.len();
        let n = row_sums[0].len();

        let mut result = vec![vec![0; n]; m];

        for col in 0..n {
            let mut sum = 0;
            for row in 0..m {
                let num = row_sums[row][col];
                sum += num;
                result[row][col] = sum;
            }
        }

        result
    }

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let row_sums = Self::make_row_sums(matrix);
        let sums = Self::make_sums(row_sums);
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let mut result = self.sums[row2][col2];
        result -= if row1 == 0 { 0 } else { self.sums[row1-1][col2] };
        result -= if col1 == 0 { 0 } else { self.sums[row2][col1-1] };
        result += if row1 == 0 || col1 == 0 { 0 } else { self.sums[row1-1][col1-1] };

        result
    }

}

#[cfg(test)]
mod tests {
    use super::NumMatrix;

    #[test]
    fn example_1() {
        let num_matrix = NumMatrix::new(vec![
            vec![3,0,1,4,2],
            vec![5,6,3,2,1],
            vec![1,2,0,1,5],
            vec![4,1,0,1,7],
            vec![1,0,3,0,5],
        ]);
        let result = num_matrix.sum_region(2,1,4,3);
        assert_eq!(result, 8);
        let result = num_matrix.sum_region(1,1,2,2);
        assert_eq!(result, 11);
        let result = num_matrix.sum_region(1,2,2,4);
        assert_eq!(result, 12);
    }

}

//  3  0  1  4  2
//  5  6  3  2  1
//  1  2  0  1  5
//  4  1  0  1  7
//  1  0  3  0  5

//  3  3  4  8 10
//  5 11 14 16 17
//  1  3  3  4  9
//  4  5  5  6 13
//  1  1  4  4  9

//  3  3  4  8 10
//  8 14 18 24 27
//  9 17 21 28 36
// 13 22 26 34 49
// 14 23 30 38 58
