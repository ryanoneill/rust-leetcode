/// There are some spherical balloons taped onto a flat wall that represents the XY-plane. The
/// balloons are represented as a 2D integer array `points` where `points[i] = [Xstart, Xend]`
/// denotes a balloon whose horizontal diameter stretches between Xstart and Xend. You do not know
/// the exact y-coordinates of the balloons.
///
/// Arrows can be shot up directly vertically (in the positive y-direction) from different points
/// along the x-axis. A balloon with `Xstart` and `Xend` is burt by an arrow shot at `x` if `Xstart
/// <= x <= Xend`. There is no limit to the number of arrows that can be shot. A shot arrow keeps
/// traveling up infinitely, bursting any balloons in its path.
struct Solution;

impl Solution {

    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable();

        let mut result = 1;

        let mut arrow_end = points[0][1];
        for point in points {
            let start = point[0];
            let end = point[1];

            if arrow_end < start {
                result += 1;
                arrow_end = end;
            } else {
                arrow_end = arrow_end.min(end);
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
        let points = vec![vec![10,16], vec![2,8], vec![1,6], vec![7,12]];
        let result = Solution::find_min_arrow_shots(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let points = vec![vec![1,2], vec![3,4], vec![5,6], vec![7,8]];
        let result = Solution::find_min_arrow_shots(points);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let points = vec![vec![1,2], vec![2,3], vec![3,4], vec![4,5]];
        let result = Solution::find_min_arrow_shots(points);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_4() {
        let points = vec![vec![9,12], vec![1,10], vec![4,11], vec![8,12], vec![3,9], vec![6,9], vec![6,7]];
        let result = Solution::find_min_arrow_shots(points);
        assert_eq!(result, 2);
    }

}
