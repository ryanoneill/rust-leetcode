/// You are given an array `coordinates`, `coordinates[i] = [x,y]`, where
/// `[x,y]` represents the coordinate of a point. Check if these points make a straight line in the
/// XY plane.
struct Solution;

impl Solution {

    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let n = coordinates.len();
        if n == 2 { true }
        else {
            let mut result = true;

            let mut last_x = coordinates[0][0];
            let mut last_y = coordinates[0][1];
            let mut current_x = coordinates[1][0];
            let mut current_y = coordinates[1][1];
            let first_diff_x = current_x - last_x;
            let first_diff_y = current_y - last_y;

            for i in 2..n {
                last_x = current_x;
                last_y = current_y;
                current_x = coordinates[i][0];
                current_y = coordinates[i][1];

                let diff_x = current_x - last_x;
                let diff_y = current_y - last_y;

                if (diff_y * first_diff_x) != (diff_x * first_diff_y) {
                    result = false;
                    break;
                }
            }

            result
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let coordinates = vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![5,6],vec![6,7]];
        let result = Solution::check_straight_line(coordinates);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let coordinates = vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]];
        let result = Solution::check_straight_line(coordinates);
        assert!(!result);
    }

}
