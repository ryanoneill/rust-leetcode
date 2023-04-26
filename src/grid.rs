use crate::direction::Direction;
use crate::position::{Position, self};

struct Grid<T> {
    squares: Vec<Vec<T>>,
    row_len: usize,
    col_len: usize,
}

impl<T> Grid<T> {

    pub fn new(squares: Vec<Vec<T>>) -> Self {
        let row_len = squares.len();
        let col_len = if row_len > 0 { squares[0].len() } else { 0 };
        Self { squares, row_len, col_len }
    }

    pub fn row_len(&self) -> usize {
        self.row_len
    }

    pub fn col_len(&self) -> usize {
        self.col_len
    }

    pub fn contains(&self, position: &Position) -> bool {
        position.row < self.row_len && position.col < self.col_len
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        if self.contains(position) {
            Some(&self.squares[position.row][position.col])
        } else { None }
    }

    pub fn move_direction(&self, position: &Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                if position.row == 0 { None }
                else { Some(Position::new(position.row - 1, position.col)) }
            }
            Direction::South => {
                if position.row == self.row_len - 1 { None }
                else { Some (Position::new(position.row + 1, position.col)) }
            }
            Direction::East => {
                if position.col == self.col_len - 1 { None }
                else { Some(Position::new(position.row, position.col + 1)) }
            }
            Direction::West => {
                if position.col == 0 { None }
                else { Some(Position::new(position.row, position.col - 1)) }
            }
        }
    }

}
