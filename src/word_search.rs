use std::collections::HashSet;

/// Given an `m x n` grid of characters `board` and a string `word`, return `true` if `word`
/// exists in the grid.
///
/// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells
/// are horizontally or vertically neighboring. The same letter cell may not be used more than
/// once.
struct Solution;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

}

impl Solution {

    fn worker(board: &Vec<Vec<char>>, letters: &Vec<char>, seen: HashSet<Position>, pos: Position, index: usize) -> bool {
        let l = letters.len();
        let mut result = false;
        let mut seen = seen;
        let directions = vec!['N', 'S', 'E', 'W'];

        if index >= l {
            result = true;
        } else {
            let m = board.len() as i32;
            let n = board[0].len() as i32;

            seen.insert(pos);
            for dir in &directions {
                let mut row = pos.row as i32;
                let mut col = pos.col as i32;
                match dir {
                    'N' => { row -= 1; }
                    'S' => { row += 1; }
                    'E' => { col += 1; }
                    'W' => { col -= 1; }
                    _   => { }
                }
                let valid_row = row >= 0 && row < m;
                let valid_col = col >= 0 && col < n;

                if valid_row && valid_col {
                    let row = row as usize;
                    let col = col as usize;
                    let letter = letters[index];
                    let value = board[row][col];
                    if letter == value {
                        let pos = Position::new(row, col);
                        if !seen.contains(&pos) {
                            result = Self::worker(board, letters, seen.clone(), pos, index+1);
                            if result {
                                break;
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut result = false;

        let letters: Vec<char> = word.chars().collect();
        let letter = letters[0];

        let m = board.len();
        let n = board[0].len();

        let seen = HashSet::new();

        for i in 0..m {
            for j in 0..n {
                let value = board[i][j];
                if value == letter {
                    let pos = Position::new(i, j);
                    result = Self::worker(&board, &letters, seen.clone(), pos, 1);
                    if result {
                        break;
                    }
                }
            }
            if result {
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
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let result = Solution::exist(board, word);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        let result = Solution::exist(board, word);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        let result = Solution::exist(board, word);
        assert!(!result);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space
// ** Input: board: grid of characters
// ** Input: word: string to find
// ** Output: true if word exists
// ** can move to adjacent cells 'N', 'S', 'E', 'W'
// ** same cell cannot be used more than once.
// ** m = board.len(), n = board[0].len()
// ** 1 <= m, n <= 6 (max of 36 squares)
// ** 1 <= word.len() <= 15 (15 * 36 = 540)
// ** board and word consists of only lowercase/uppercase English letters.
//
// 3. Write down the test cases
// ** Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// ** Output: true
//
// ** Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// ** Output: true
//
// ** Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// ** Output: false
//
// 4. Describe and write down the algorithm
// ** Start at squares which match first character of word. If none match, return false
// ** For each of those, run a search starting there if the last one was unsuccessful.
// **   Within search, keep track of visisted positions
// **   Look N, S, E, W if any of those equal the next letter and not visited yet, move there with
// **   If no more ways to go, stop
// **   If end of the word and made it to the last letter, return true
// ** 
// ** Space Complexity: O(L) for DFS using recursion
// ** Time Complexity: O(N * 4^L) since N/S/E/W but one is always where you came from so
// **                  O(N * 3^L)
