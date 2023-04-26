use std::collections::HashSet;

struct GridMarker {
    seen: HashSet<(usize, usize)>
}

impl GridMarker {

    pub fn new() -> Self {
        GridMarker { seen: HashSet::new() }
    }

    pub fn is_marked(&self, row: usize, col: usize) -> bool {
        self.seen.contains(&(row, col))
    }

    pub fn mark(&mut self, row: usize, col: usize) {
        self.seen.insert((row, col));
    }

}
