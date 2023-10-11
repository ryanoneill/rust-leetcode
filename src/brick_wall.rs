use std::collections::HashMap;

/// There is a rectangular brick wall in front of you with `n` rows of bricks. The `ith` row has
/// some number of bricks each of the same height (i.e., one unit) but they can be of different
/// widths. The total width of each row is the same. 
///
/// Draw a vertical line from the top to the bottom and cross the least bricks. If your line goes
/// through the edge of a brick, then the brick is not considered as crossed. You cannot draw a
/// line just along one of the two vertical edges of the wall, in which case the line will
/// obviously cross no bricks.
///
/// Given the 2D array `wall` that contains the information about the wall, return the mininum
/// number of crossed bricks after drawing such a vertical line.
struct Solution;

impl Solution {

    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut breaks = HashMap::new();
        for row in &wall {
            let mut sum = 0;
            let n = row.len();
            for i in 0..n-1 {
                sum += row[i];
                breaks
                    .entry(sum)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let n = wall.len();
        let max = breaks.values().max().copied().unwrap_or(0);
        (n as i32) - max
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let wall = vec![vec![1,2,2,1], vec![3,1,2], vec![1,3,2], vec![2,4], vec![3,1,2], vec![1,3,1,1]];
        let result = Solution::least_bricks(wall);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let wall = vec![vec![1], vec![1], vec![1]];
        let result = Solution::least_bricks(wall);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let wall = vec![vec![1,1], vec![2], vec![1,1]];
        let result = Solution::least_bricks(wall);
        assert_eq!(result, 1);
    }

}
