/// Given a circular integer array `nums` of length `n`, return the maximum possible sum of a
/// non-empty subarray of `nums`.
///
/// A circular array means the end of the array connects to the beginning of the array. Formally,
/// the next element of `nums[i]` is `nums[(i + 1) % n]` and the previous element of `nums[i]` is
/// `nums[(i - 1 + n) % n]`.
///
/// A subarray may only include each element of the fixed buffer `nums` at most once. Formally, for
/// a subarray `nums[i], nums[i + 1], ..., nums[j]`, there does not exist `i <= k1, k2 <= j with k1
/// % n == k2 % n`.
struct Solution;

impl Solution {

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;

        let mut max_sum = i32::MIN;
        let mut current_max_sum = 0;

        let mut min_sum = i32::MAX;
        let mut current_min_sum = 0;

        for num in nums {
            total_sum += num;

            if current_max_sum < 0 {
                current_max_sum = 0;
            }
            current_max_sum += num;
            max_sum = max_sum.max(current_max_sum);

            if current_min_sum > 0 {
                current_min_sum = 0;
            }
            current_min_sum += num;
            min_sum = min_sum.min(current_min_sum);
        }

        if max_sum < 0 {
            max_sum
        } else {
            max_sum.max(total_sum - min_sum)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,-2,3,-2];
        let result = Solution::max_subarray_sum_circular(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![5,-3,5];
        let result = Solution::max_subarray_sum_circular(nums);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_3() {
        let nums = vec![-3,-2,-3];
        let result = Solution::max_subarray_sum_circular(nums);
        assert_eq!(result, -2);
    }

}
