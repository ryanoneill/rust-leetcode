/// You are given a 0-indexed integer array `nums`. In one operation you can replace any element of
/// the array with any two elements that sum to it.
///
/// * For example, consider `nums = [5,6,7]`. In one operation, we can replace `nums[1]` with `2`
/// and `4` and convert `nums` to `[5,2,4,7]`.
///
/// Return the minimum number of operations to make an array that is sorted in non-decreasing
/// order.
struct Solution;

impl Solution {

    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut result = 0;
        let n = nums.len();

        for i in (0..n-1).rev() {
            let before = nums[i];
            let after = nums[i + 1];

            if before > after {
                let elems = (before + after - 1) / after;

                result += (elems - 1) as i64;

                nums[i] = before / elems;
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
        let nums = vec![3,9,3];
        let result = Solution::minimum_replacement(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,5];
        let result = Solution::minimum_replacement(nums);
        assert_eq!(result, 0);
    }

}
