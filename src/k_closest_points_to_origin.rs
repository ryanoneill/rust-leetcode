use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct State {
    distance: f64,
    x: i32,
    y: i32,
}

impl State {

    fn new(x: i32, y: i32) -> Self {
        let distance = Self::from_origin(x, y);
        State { distance, x, y }
    }

    fn from_origin(x: i32, y: i32) -> f64 {
        let x = x as f64;
        let y = y as f64;
        let distance = ((x * x) + (y * y)).sqrt();
        distance
    }

    fn to_points(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }

}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance { Ordering::Less }
        else if self.distance > other.distance { Ordering::Greater }
        else if self.x < other.x { Ordering::Less }
        else if self.x > other.x { Ordering::Greater }
        else if self.y < other.y { Ordering::Less }
        else if self.y > other.y { Ordering::Greater }
        else { Ordering::Equal }
    }
}

impl Eq for State { }

/// Given an array of `points` where `points[i] = [xi, yi]` represents a point
/// on the X-Y plane and an integer `k`, return the `k` closest points to the
/// origin `(0, 0)`.
///
/// The distance between two points on the X-Y plane is the Euclidean distance
/// (i.e., `sqrt((x1 - x2)^2 + (y1 - y2)^2)`).
///
/// You may return the answer in any order. The answer is guaranteed to be
/// unique (except for the order that it is in).
struct Solution;

impl Solution {

    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap = BinaryHeap::new();
        for point in points {
            let state = State::new(point[0], point[1]);
            min_heap.push(Reverse(state));
        }
        let mut result = Vec::new();
        for _ in 0..k as usize {
            let item = min_heap.pop().unwrap().0;
            result.push(item.to_points());
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let points = vec![vec![1,3], vec![-2,2]];
        let k = 1;
        let result = Solution::k_closest(points, k);
        assert_eq!(result, vec![vec![-2,2]]);
    }

    #[test]
    fn example_2() {
        let points = vec![vec![3,3], vec![5,-1], vec![-2,4]];
        let k = 2;
        let result = Solution::k_closest(points, k);
        assert_eq!(result, vec![vec![3,3], vec![-2,4]]);
    }

}
