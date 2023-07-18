/// The median is the middle value in an ordered integer list. If the size of the list is even,
/// there is no middle value. So the median is the mean of the two middle values.
///
/// * For examples, if `arr = [2,3,4]`, the median is `3`.
///
/// * For examples, if `arr = [1,2,3,4]`, the median is `(2 + 3) / 2 = 2.5`.
///
/// You are given an integer array `nums` and an integer `k`. There is a sliding window of size `k`
/// which is moving from the very left of the array to the very right. You can only see the `k`
/// numbers in the window. Each time the sliding window moves right by one position.
///
/// Return the median array for each window in the original array. Answers within `10^-5` of the
/// actual value will be accepted.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn median_sliding_window(_nums: Vec<i32>, _k: i32) -> Vec<f64> {
        vec![]
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let k = 3;
        let result = Solution::median_sliding_window(nums, k);
        assert_eq!(result, vec![1.0,-1.0,-1.0,3.0,5.0,6.0]);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,2,3,1,4,2];
        let k = 3;
        let result = Solution::median_sliding_window(nums, k);
        assert_eq!(result, vec![2.0,3.0,3.0,3.0,2.0,3.0,2.0]);
    }

}


