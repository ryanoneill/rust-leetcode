use std::cmp::max;

/// You are given an integer array `height` of length `n`. There are `n`
/// vertical lines drawn such that the two endpoints of the `ith` line are
/// `(i, 0)` and `(i, height[i])`.
///
/// Find two lines that together with the x-axis form a container, such that
/// the container contains the most water.
///
/// Return the maximum amount of water a container can store.
///
/// Notice that you may not slant the container.
struct Solution;

impl Solution {

    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_area = 0;
        let mut left: usize = 0;
        let mut right: usize = n - 1;

        while left < right {
            let left_height = height[left];
            let right_height = height[right];
            let width = (right - left) as i32;
            if left_height < right_height {
                let area = width * left_height;
                max_area = max(max_area, area);
                left += 1;
            } else {
                let area = width * right_height;
                max_area = max(max_area, area);
                right -= 1;
            }
        }

        max_area
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn example_2() {
        let height = vec![1,1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }

}
