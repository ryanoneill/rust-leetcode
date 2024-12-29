use std::iter::Flatten;
use std::iter::Peekable;
use std::vec::IntoIter;

/// Design an iterator to flatten a 2D vector. It should support the
/// `next` and `hasNext` operations.
///
/// Implement the `Vector2D` class:
///
/// * `Vector2D(int[][] vec)` initializes the object with the 2D vector `vec`.
///
/// * `next()` returns the next element from the 2D vector and moves the pointer one step forward.
///   You may assume that all the calls to `next` are valid.
///
/// * `hasNext()` returns `true` if there are still some elements in the vector, and `false`
///   otherwise.
struct Vector2D {
    iter: Peekable<Flatten<IntoIter<Vec<i32>>>>,
} 

impl Vector2D {

    fn new(vec: Vec<Vec<i32>>) -> Self {
        Self { 
            iter: vec.into_iter().flatten().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.iter.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.iter.peek().is_some()
    }

}

#[cfg(test)]
mod tests {
    use super::Vector2D;

    #[test]
    fn example_1() {
        let items = vec![vec![1,2], vec![3], vec![4]];
        let mut vector_2d = Vector2D::new(items);
        let next = vector_2d.next();
        assert_eq!(next, 1);
        let next = vector_2d.next();
        assert_eq!(next, 2);
        let next = vector_2d.next();
        assert_eq!(next, 3);
        let has_next = vector_2d.has_next();
        assert!(has_next);
        let has_next = vector_2d.has_next();
        assert!(has_next);
        let next = vector_2d.next();
        assert_eq!(next, 4);
        let has_next = vector_2d.has_next();
        assert!(!has_next);
    }

    #[test]
    fn example_2() {
        let items = vec![vec![],vec![3],vec![],vec![],vec![]]; 
        let mut vector_2d = Vector2D::new(items);
        let has_next = vector_2d.has_next();
        assert!(has_next);
        let next = vector_2d.next();
        assert_eq!(next, 3);
        let has_next = vector_2d.has_next();
        assert!(!has_next);
    }

}
