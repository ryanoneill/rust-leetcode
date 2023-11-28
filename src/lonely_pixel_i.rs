/// Given an `m x n` `picture` consisting of black `'B'` and white `'W'` pixels, return the number
/// of black lonely pixels.
///
/// A black lonely pixel is a character `'B'` that located at a specific position where the same
/// row and same column don't have any other black pixels.
struct Solution;

impl Solution {

    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        let m = picture.len();
        let n = picture[0].len();

        let mut rows = vec![0; m];
        let mut cols = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                let pixel = picture[i][j];
                if pixel == 'B' {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        let mut result = 0;
        for i in 0..m {
            let row = rows[i];
            if row == 1 {
                for j in 0..n {
                    let col = cols[j];
                    if col == 1 && picture[i][j] == 'B' {
                        result += 1;
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
        let picture = vec![vec!['W', 'W', 'B'], vec!['W', 'B', 'W'], vec!['B', 'W', 'W']];
        let result = Solution::find_lonely_pixel(picture);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let picture = vec![vec!['B', 'B', 'B'], vec!['B', 'B', 'W'], vec!['B', 'B', 'B']];
        let result = Solution::find_lonely_pixel(picture);
        assert_eq!(result, 0);
    }

}
