use crate::position::Position;
use std::collections::HashSet;

struct GridMarker {
    seen: HashSet<Position>
}

impl GridMarker {

    pub fn new() -> Self {
        GridMarker { seen: HashSet::new() }
    }

    pub fn is_marked(&self, position: &Position) -> bool {
        self.seen.contains(position)
    }

    pub fn mark(&mut self, position: Position) {
        self.seen.insert(position);
    }

}
