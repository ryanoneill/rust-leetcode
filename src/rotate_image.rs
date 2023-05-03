/// You are given an `n x n` 2D `matrix` representing an image, rotate the
/// image by 90 degrees (clockwise).
///
/// You have to rotate the image in-place, which means you have to modify the
/// input 2D matrix directly. DO NOT allocate another 2D matrix and do the
/// rotation.
struct Solution;

impl Solution {

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = if n % 2 == 0 { n / 2 } else { (n / 2) + 1 };
        let mut temp;

        for i in 0..m {
            for j in 0..n/2 {
                println!("{} {}", i, j);
                let back_i = n - i - 1;
                let back_j = n - j - 1;

                temp = matrix[i][j];
                matrix[i][j] = matrix[back_j][i];
                matrix[back_j][i] = matrix[back_i][back_j];
                matrix[back_i][back_j] = matrix[j][back_i];
                matrix[j][back_i] = temp;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
    }

    #[test]
    fn example_2() {
        let mut matrix = vec![vec![5,1,9,11], vec![2,4,8,10], vec![13,3,6,7], vec![15,14,12,16]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![15,13,2,5], vec![14,3,4,1], vec![12,6,8,9], vec![16,7,10,11]]);
    }

    #[test]
    fn five_by_five() {
        let mut matrix = vec![vec![1,2,3,4,5],vec![6,7,8,9,10],vec![11,12,13,14,15],vec![16,17,18,19,20],vec![21,22,23,24,25]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![21,16,11,6,1],vec![22,17,12,7,2],vec![23,18,13,8,3],vec![24,19,14,9,4],vec![25,20,15,10,5]]);
    }

}
