use crate::union_find::UnionFind;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    index: i32,
    x: i32,
    y: i32,
}

impl Point {

    fn new(index: i32, x: i32, y: i32) -> Self {
        Self { index, x, y }
    }

}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Path {
    distance: i32,
    from: Point,
    to: Point,
}

impl Path {

    fn new(from: Point, to: Point) -> Self {
        Self {
            from,
            to,
            distance: (to.x - from.x).abs() + (to.y - from.y).abs()
        }
    }

}

/// You are given an array `points` representing integer coordinates of some
/// points on a 2D-plane, where `points[i] = [xi, yi]`.
///
/// The cost of connecting two points `[xi, yi]` and `[xj, yj]` is the
/// manhattan distance between them: `|xi - xj| + |yi - yj|`, where |val|
/// denotes the absolute value of `val`.
///
/// Return the minimum cost to make all points connected. All points are
/// connected if there is exactly one simple path between any two points.
struct Solution;

impl Solution {

    fn to_points(points: &Vec<Vec<i32>>) -> Vec<Point> {
        let mut result = Vec::new();
        let n = points.len();

        for i in 0..n {
            let point = Point::new(i as i32, points[i][0], points[i][1]);
            result.push(point);
        }
        result
    }

    fn to_paths(points: &Vec<Point>) -> Vec<Path> {
        let n = points.len();
        let mut result = Vec::new();
        for i in 0..n {
            for j in i+1..n {
                let from = points[i];
                let to = points[j];
                let path = Path::new(from, to);
                result.push(path);
            }
        }
        result
    }

    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let points = Self::to_points(&points);
        let mut paths = Self::to_paths(&points);
        paths.sort();

        let n = points.len();
        let mut uf = UnionFind::new(n);
        let mut included = Vec::new();

        for path in paths {
            let x = path.from.index;
            let y = path.to.index;
            if !uf.connected(x, y) {
                included.push(path);
                uf.union(x, y);
            }
        }

        included.iter().map(|p| p.distance).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let points = vec![vec![0,0], vec![2,2], vec![3,10], vec![5,2], vec![7,0]];
        let result = Solution::min_cost_connect_points(points);
        assert_eq!(result, 20);
    }

    #[test]
    fn example_2() {
        let points = vec![vec![3,12], vec![-2,5], vec![-4,1]];
        let result = Solution::min_cost_connect_points(points);
        assert_eq!(result, 18);
    }

}
