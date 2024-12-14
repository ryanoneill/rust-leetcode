/// You are given an array representing a row of `seats` where `seats[i] = 1` represents a person
/// sitting in the `ith` seat, and `seats[i] == 0` represents the `ith` seat is empty (0-indexed).
///
/// There is at least one empty seat, and at least one person sitting.
///
/// Alex wants to sit in the seat such that the distance between him and the closest person to him
/// is maximized.
///
/// Return that maximum distance to the closest person.
struct Solution;

impl Solution {

    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();

        let mut result = 0;
        let mut last_filled;

        let mut i = 0;
        loop {
            if seats[i] == 1 {
                last_filled = i;
                if i > 0 {
                    result = i;
                }
                break;
            }
            i += 1;
        }

        for j in i..n {
            if seats[j] == 1 {
                let diff = (j - last_filled) / 2;
                if diff > result {
                    result = diff;
                }
                last_filled = j;
            }
        }

        if seats[n-1] == 0 {
            let diff = n-1 - last_filled;
            if diff > result {
                result = diff;
            }
        }

        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let seats = vec![1,0,0,0,1,0,1];
        let result = Solution::max_dist_to_closest(seats);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let seats = vec![1,0,0,0];
        let result = Solution::max_dist_to_closest(seats);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let seats = vec![0,1];
        let result = Solution::max_dist_to_closest(seats);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let seats = vec![0,0,0,1,0,1,0];
        let result = Solution::max_dist_to_closest(seats);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_5() {
        let seats = vec![0,1,0,0,0,0,0,0,1,0,0,1];
        let result = Solution::max_dist_to_closest(seats);
        assert_eq!(result, 3);
    }

}
