#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

impl Position {

    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

}
