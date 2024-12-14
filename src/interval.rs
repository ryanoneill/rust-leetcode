#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Interval {
    pub start: i32,
    pub end: i32
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end
        }
    }
}
