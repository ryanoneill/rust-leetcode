/// Determine if a `9 x 9` Sudoku board is valid. Only the filled cells need to
/// be validated according to the following rules:
///
/// Each row must contain the digits `1-9` without repetition.
///
/// Each column must contain the digits `1-9` without repetition.
///
/// Each of the nine `3 x 3` sub-boxes of the grid must contain the digits `1-9`
/// without repetition.
///
/// Note:
/// * A Sudoku board (partially filled) could be valid but is not necessarily
///   solvable.
///
/// * Only the filled cells need to be validated according to the mentioned
///   rules.
struct Solution;

impl Solution {

    fn char_to_index(c: char) -> usize {
        c.to_digit(10).unwrap_or_default() as usize
    }

    fn is_valid_component<F>(board: &Vec<Vec<char>>, f: F) -> bool
        where F: Fn(&Vec<Vec<char>>, usize) -> char {

        let mut result = true;
        let mut working = vec![0; 10];
        for i in 0..9 {
            let value = f(board, i);
            if value != '.' {
                let w = Self::char_to_index(value);
                if working[w] == w {
                    result = false;
                    break;
                } else { working[w] = w; }
            }
        }

        result
    }

    fn is_valid_section(board: &Vec<Vec<char>>, sec: usize) -> bool {
        let row_offset = match sec {
            0 | 1 | 2 => 0,
            3 | 4 | 5 => 3,
            6 | 7 | 8 => 6,
            _ => 0,
        };
        let col_offset = match sec {
            0 | 3 | 6 => 0,
            1 | 4 | 7 => 3,
            2 | 5 | 8 => 6,
            _ => 0,
        };

        Self::is_valid_component(board, |board, num| {
            let (row, col) = match num {
                0 => (0 + row_offset, 0 + col_offset),
                1 => (0 + row_offset, 1 + col_offset),
                2 => (0 + row_offset, 2 + col_offset),
                3 => (1 + row_offset, 0 + col_offset),
                4 => (1 + row_offset, 1 + col_offset),
                5 => (1 + row_offset, 2 + col_offset),
                6 => (2 + row_offset, 0 + col_offset),
                7 => (2 + row_offset, 1 + col_offset),
                8 => (2 + row_offset, 2 + col_offset),
                _ => (0, 0)
            };
            board[row][col]
        })
    }

    fn is_valid_column(board: &Vec<Vec<char>>, col: usize) -> bool {
        Self::is_valid_component(board, |board, row| board[row][col])
    }

    fn is_valid_row(board: &Vec<Vec<char>>, row: usize) -> bool {
        Self::is_valid_component(board, |board, col| board[row][col])
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut result = true;

        for i in 0..9 {
            result = result && Self::is_valid_row(&board, i);
            result = result && Self::is_valid_column(&board, i);
            result = result && Self::is_valid_section(&board, i);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        let result = Solution::is_valid_sudoku(board);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        let result = Solution::is_valid_sudoku(board);
        assert!(!result);
    }

}
