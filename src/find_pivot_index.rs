/// Given an array of integers `nums`, calculate the pivot index of this array.
///
/// The pivot index is the index where the sum of all the numbers strictly to
/// the left of the index is equal to the sum of all the numbers strictly to
/// the index's right.
///
/// If the index is on the left edge of the array, then the left sum is `0`
/// because there are no elements to the left. This also applies to the right
/// edge of the array.
///
/// Return the leftmost pivot index. If no such index exists, return `-1`.
struct Solution;

impl Solution {

    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];

        let mut sum = 0;
        left_sum[0] = sum;
        for i in 0..n-1 {
            let value = nums[i];
            sum += value;
            left_sum[i+1] = sum;
        }

        let mut sum = 0;
        right_sum[n-1] = sum;
        for i in 0..n-1 {
            let j = n-1-i;
            let value = nums[j];
            sum += value;
            right_sum[j-1] = sum;
        }

        let mut result = -1;
        for i in 0..n {
            if left_sum[i] == right_sum[i] {
                result = i as i32;
                break;
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
        let nums = vec![1,7,3,6,5,6];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let nums = vec![2,1,-1];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, 0);
    }

}
