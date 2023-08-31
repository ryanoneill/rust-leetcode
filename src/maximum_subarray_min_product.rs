#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    start: usize,
    value: i32,
}

impl Item {

    pub fn new(start: usize, value: i32) -> Self {
        Self { start, value }
    }

}


/// The min-product of an array is equal to the minimum value in the array multiplied by the
/// array's sum.
///
/// * For example, the array `[3,2,5]` (minimum value `2`) has a min-product of `2 * (3+2+5)` = 2 *
/// 10 = 20.
///
/// Given an array of integers `nums`, return the maximum min-product of any non-empty subarray of
/// `nums`. Since the answer may be large, return it modulo `10^9 + 7`.
///
/// Note that the min-product should be maximized before performing the modulo operation. Testcases
/// are generated such that the maximum min-product without modulo will fit in a 64-bit signed
/// integer.
///
/// A subarray is a contiguous part of an array.
struct Solution;

impl Solution {

    fn last_is_greater_than(stack: &Vec<Item>, value: i32) -> bool {
        if !stack.is_empty() {
            let item = stack.last().unwrap();
            item.value > value
        } else {
            false
        }
    }

    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let mod_value: i64 = 10_i64.pow(9) + 7;

        let mut result: i64 = 0;
        let mut stack = Vec::new();

        let mut prefix: Vec<i64> = vec![0];
        let n = nums.len();
        for i in 0..n {
            let num = nums[i] as i64;
            let previous = prefix[i];
            let total = num + previous;
            prefix.push(total);
        }

        for (i, &num) in nums.iter().enumerate() {
            let mut start = i;

            while Self::last_is_greater_than(&stack, num) {
                let item = stack.pop().unwrap();
                let total = prefix[i] - prefix[item.start];
                result = result.max(item.value as i64 * total);
                start = item.start;
            }
            stack.push(Item::new(start, num));
        }

        for item in stack {
            let total = prefix[n] - prefix[item.start];
            result = result.max(item.value as i64 * total);
        }

        (result % mod_value) as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,2];
        let result = Solution::max_sum_min_product(nums);
        assert_eq!(result, 14);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,3,3,1,2];
        let result = Solution::max_sum_min_product(nums);
        assert_eq!(result, 18);
    }

    #[test]
    fn example_3() {
        let nums = vec![3,1,5,6,4,2];
        let result = Solution::max_sum_min_product(nums);
        assert_eq!(result, 60);
    }

}
