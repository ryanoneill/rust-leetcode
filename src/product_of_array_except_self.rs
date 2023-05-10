/// Given an integer array `nums`, return an array `answer` such that
/// `answer[i]` is equal to the product of all the elements of `nums` except
/// `nums[i]`.
///
/// The product of any prefix or suffix of `nums` is guaranteed to fit in a
/// 32-bit integer.
///
/// You must write an algorithm that runs in `O(n)` time and without using the
/// division operation.
struct Solution;

impl Solution {

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut result = vec![0; n];

        let mut product = 1;
        left[0] = product;
        for i in 0..n-1 {
            let value = nums[i];
            product *= value;
            left[i+1] = product;
        }

        product = 1;
        right[n-1] = product;
        for i in 0..n-1 {
            let value = nums[n-1-i];
            product *= value;
            right[n-i-2] = product;
        }

        for i in 0..n {
            result[i] = left[i] * right[i];
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,4];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24,12,8,6]);
    }

    #[test]
    fn exmaple_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn two_zeroes() {
        let nums = vec![0, 1, 1, 0];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }

    #[test]
    fn two_nums() {
        let nums = vec![-30, 30];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![30, -30]);
    }

}
