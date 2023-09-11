/// On a 2D plane, there are `n` points with integer coordinates `points[i] = [xi, yi]`. Return the
/// minimum time in seconds to visit all points in the order given by `points`.
///
/// You can move according to these rules:
/// * In `1` second, you can either:
///   * move vertically by one unit,
///   * move horizontally by one unit,
///   * move diagonally `sqrt(2)` units (in other words, move one unit vertically then one unit
///     horizontally in `1` second).
/// * You have to visit the points in the same order as they appear in the array.
/// * You are allowed to pass through points that appear later in the order, but these do not count
///   as visits.
struct Solution;

impl Solution {

    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for pair in points.windows(2) {
            result += (pair[0][0] - pair[1][0]).abs().max((pair[0][1] - pair[1][1]).abs())
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let points = vec![vec![1,1], vec![3,4], vec![-1,0]];
        let result = Solution::min_time_to_visit_all_points(points);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let points = vec![vec![3,2], vec![-2,2]];
        let result = Solution::min_time_to_visit_all_points(points);
        assert_eq!(result, 5);
    }

}
