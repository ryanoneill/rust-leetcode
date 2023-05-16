/// Given an integer array `nums`, return the maximum difference between two
/// successive elements in its sorted form. If the array contains less than two
/// elements, return `0`.
///
/// You must write an algorithm that runs in linear time and uses linear space.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn maximum_gap(_nums: Vec<i32>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let nums = vec![3,6,9,1];
        let result = Solution::maximum_gap(nums);
        assert_eq!(result, 3);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let nums = vec![10];
        let result = Solution::maximum_gap(nums);
        assert_eq!(result, 0);
    }

}
