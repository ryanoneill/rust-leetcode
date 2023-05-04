/// You are given an integer array `heights` representing the heights of
/// buildings, some `bricks`, and some `ladders`.
///
/// You start your journey from building `0` and move to the next building by
/// possibly using bricks or ladders.
///
/// While moving from building `i` to building `i+1` (0-indexed),
///
/// * If the current building's height is greater than or equal to the next
///   building's height, you do not need a ladder or bricks.
///
/// * If the current building's height is less than the next building's height,
///   you can either use one ladder or `(h[i+1] - h[i])` bricks.
///
/// Return the furthest building index (0-indexed) you can reach if you use the
/// given ladders and bricks optimally.
struct Solution;

impl Solution {
    fn prefix_diff(n: usize, heights: &Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut previous = heights[0];
        for i in 1..n {
            let current = heights[i];
            let diff = current - previous;
            if diff > 0 {
                result.push(diff);
            }
            previous = current;
        }
        result
    }

    fn bricks_for_buildings(heights: &Vec<i32>, bricks: i32) -> i32 {
        let mut bricks = bricks;
        let mut result = 0;

        for height in heights {
            if bricks >= *height {
                bricks -= height;
                result += 1;
            } else {
                break;
            }
        }

        result
    }

    fn check_furthest(n: usize, heights: &Vec<i32>, bricks: i32, ladders: i32) -> bool {
        let mut diffs = Self::prefix_diff(n, heights);
        diffs.sort();
        let brick_buildings = Self::bricks_for_buildings(&diffs, bricks);
        let total = brick_buildings + ladders;
        total >= diffs.len() as i32
    }

    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let n = heights.len();
        let result: i32;

        if Self::check_furthest(n, &heights, bricks, ladders) {
            result = n as i32;
        } else {
            let mut last_bad = n;
            let mut last_good = 1;

            while last_bad - last_good > 1 {
                let mid = last_good + (last_bad - last_good) / 2;
                if Self::check_furthest(mid, &heights, bricks, ladders) {
                    last_good = mid;
                } else {
                    last_bad = mid;
                }
            }
            result = last_good as i32;
        }

        result - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let bricks = 5;
        let ladders = 1;
        let result = Solution::furthest_building(heights, bricks, ladders);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let bricks = 10;
        let ladders = 2;
        let result = Solution::furthest_building(heights, bricks, ladders);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_3() {
        let heights = vec![14, 3, 19, 3];
        let bricks = 17;
        let ladders = 0;
        let result = Solution::furthest_building(heights, bricks, ladders);
        assert_eq!(result, 3);
    }
}
