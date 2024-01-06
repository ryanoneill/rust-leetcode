/// The n-queens puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no
/// two queens attack each other.
///
/// Given an integer `n`, return all distinct solutions to the n-queens puzzle. You may return the
/// answer in any order.
struct Solution;

impl Solution {

    fn contains(value: u16, diagonal: u16) -> bool {
        let mask = 1 << diagonal;
        value & mask == mask
    }

    fn not_contains(value: u16, diagonal: u16) -> bool {
        !Self::contains(value, diagonal)
    }

    fn insert(value: &mut u16, diagonal: u16) {
        let mask = 1 << diagonal;
        *value |= mask;
    }

    fn backtrack(
        n: u16,
        row: u16,
        diagonals: u16,
        anti_diagonals: u16,
        columns: u16
    ) -> i32 {
        let mut result = 0;
        if row == n {
            result = 1;
        } else {
            for col in 0..n {
                let diagonal = n + row - col;
                let anti_diagonal = row + col;

                let good = Self::not_contains(columns, col)
                    && Self::not_contains(diagonals, diagonal)
                    && Self::not_contains(anti_diagonals, anti_diagonal);

                if good {
                    let mut columns = columns;
                    let mut diagonals = diagonals;
                    let mut anti_diagonals = anti_diagonals;

                    Self::insert(&mut columns, col);
                    Self::insert(&mut diagonals, diagonal);
                    Self::insert(&mut anti_diagonals, anti_diagonal);

                    result += Self::backtrack(n, row + 1, diagonals, anti_diagonals, columns);
                }
            }
        }

        result
    }

    pub fn total_n_queens(n: i32) -> i32 {
        Self::backtrack(n as u16, 0, 0, 0, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1()  {
        let n = 4;
        let result = Solution::total_n_queens(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 1;
        let result = Solution::total_n_queens(n);
        assert_eq!(result, 1);
    }

}
