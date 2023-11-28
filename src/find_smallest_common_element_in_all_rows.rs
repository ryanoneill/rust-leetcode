/// Given an `m x n` matrix `mat` where every row is sorted in strictly increasing order, return
/// the smallest common element in all rows.
///
/// If there is no common element, return `-1`.
struct Solution;

impl Solution {

    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; 10001]; // 1 <= mat[i][j] <= 10000
        let m = mat.len();
        let n = mat[0].len();

        let mut result = -1;

        for col in 0..n {
            for row in 0..m {
                let index = mat[row][col] as usize;
                counts[index] += 1;
                if counts[index] == m {
                    result = index as i32;
                    break;
                }
            }
            if result != -1 {
                break;
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
        let mat = vec![
            vec![1,2,3,4,5],
            vec![2,4,5,8,10],
            vec![3,5,7,9,11],
            vec![1,3,5,7,9],
        ];
        let result = Solution::smallest_common_element(mat);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let mat = vec![
            vec![1,2,3],
            vec![2,3,4],
            vec![2,3,5],
        ];
        let result = Solution::smallest_common_element(mat);
        assert_eq!(result, 2);
    }

}
