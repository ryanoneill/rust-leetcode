/// Given `n` non-negative integers representing an elevation map where the width of each bar is
/// `1`, compute how much water it can trap after raining.
struct Solution;

impl Solution {

    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 { 0 }
        else {
            let mut result = 0;

            let mut left = 0;
            let mut right = height.len() - 1;
            let mut left_max = height[left];
            let mut right_max = height[right];

            while left < right {
                if left_max < right_max {
                    left += 1;
                    left_max = left_max.max(height[left]);
                    result += left_max - height[left];
                } else {
                    right -= 1;
                    right_max = right_max.max(height[right]);
                    result += right_max - height[right];
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
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let result = Solution::trap(height);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let height = vec![4,2,0,3,2,5];
        let result = Solution::trap(height);
        assert_eq!(result, 9);
    }

}
