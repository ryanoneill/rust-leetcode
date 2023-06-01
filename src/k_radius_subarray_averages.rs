/// You are given a 0-indexed array `nums` of `n` integers, and an integer `k`.
///
/// The k-radius average for a subarray of `nums` centered at some index `i` with the radius `k` is
/// the average of all elements in `nums` between the inidices of `i - k` and `i + k` (inclusive).
/// If there are less than `k` elements before or after the index `i`, then the k-radius average is
/// `-1`.
///
/// Build and return an array `avgs` of length `n` where `avgs[i]` is the k-radius average for the
/// subarray centered at index `i`.
///
/// The average of `x` elements is the sum of the `x` elements divided by `x`, using integer
/// division. The integer division truncates toward zero, which means losing its fractional part.
///
/// * For example, the average of four elements `2`, `3`, `1`, and `5` is
///   `(2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75`, which truncates to `2`.
struct Solution;

impl Solution {

    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let two_k = 2 * k;
        let n = nums.len();
        let width = 2 * k + 1;
        let mut result = vec![-1; n];

        if width <= n {
            let mut sum: i64 = 0;

            for i in 0..two_k{
                let num = nums[i];
                sum += num as i64;
            }

            for i in two_k..n {
                let num = nums[i];
                sum += num as i64;
                result[i - k] = (sum / (width as i64)) as i32;

                let removed = nums[i - two_k] as i64;
                sum -= removed;
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
        let nums = vec![7,4,3,9,1,8,5,2,6];
        let k = 3;
        let result = Solution::get_averages(nums, k);
        assert_eq!(result, vec![-1,-1,-1,5,4,4,-1,-1,-1]);
    }

    #[test]
    fn example_2() {
        let nums = vec![100000];
        let k = 0;
        let result = Solution::get_averages(nums, k);
        assert_eq!(result, vec![100000]);
    }

    #[test]
    fn example_3() {
        let nums = vec![8];
        let k = 100000;
        let result = Solution::get_averages(nums, k);
        assert_eq!(result, vec![-1]);
    }

    #[test]
    fn extra() {
        let nums = vec![1,2,3,4,5,6,7,8];
        let k = 1;
        let result = Solution::get_averages(nums, k);
        assert_eq!(result, vec![-1,2,3,4,5,6,7,-1]);
    }

}
