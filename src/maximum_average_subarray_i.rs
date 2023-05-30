/// You are given an integer array `nums` consisting of `n` elements, and an
/// integer `k`.
///
/// Find a contiguous subarray whose length is euqal to `k` that has the
/// maximum average value and return this value.
struct Solution;

impl Solution {

    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let k = k as usize;
        let mut start = 0;
        let mut end = start + k - 1;

        let mut sum = 0;

        for i in start..=end {
            sum += nums[i] as i64;
        }
        let mut max = sum;

        end += 1;
        while end < n {
            sum -= nums[start] as i64;
            start += 1;
            sum += nums[end] as i64;
            end += 1;

            max = max.max(sum);
        }

        (max as f64) / (k as f64)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,12,-5,-6,50,3];
        let k = 4;
        let result = Solution::find_max_average(nums, k);
        assert_eq!(result, 12.75);
    }

    #[test]
    fn example_2() {
        let nums = vec![5];
        let k = 1;
        let result = Solution::find_max_average(nums, k);
        assert_eq!(result, 5.0);
    }

}
