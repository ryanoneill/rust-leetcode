/// Given an integer array `nums`, find a subarray that has the largest product, and return the
/// product.
///
/// The test cases are generated so that the answer will fit in a 32-bit integer.
struct Solution;

impl Solution {

    pub fn max_product(_nums: Vec<i32>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let nums = vec![2,3,-2,4];
        let result = Solution::max_product(nums);
        assert_eq!(result, 6);
    }


    // TODO: Add More Tests

}
