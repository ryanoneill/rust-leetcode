/// You are given a 0-indexed array `nums` of distinct integers. You want to
/// rearrange the elements in the array such that every element in the
/// rearranged array is not equal to the average of its neighbors.
///
/// More formally, the rearranged array should have the property that for
/// every `i` in the range `1 <= i < nums.length - 1`,
/// `(nums[i-1] + nums[i+1]) / 2` is not equal to `nums[i]`.
///
/// Return any rearrangement of `nums` that meets the requirements.
struct Solution;

impl Solution {
    // TODO: Improve O(n log n) running time
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut result = vec![0; n];

        let mid = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
        for i in 0..mid {
            result[i * 2] = nums[i];
        }
        for i in mid..n {
            result[(i - mid) * 2 + 1] = nums[i];
        }

        result
    }

    pub fn is_valid(nums: Vec<i32>) -> bool {
        let mut result = true;
        for i in 1..nums.len() - 1 {
            let num = nums[i];
            let avg = (nums[i - 1] + nums[i + 1]) / 2;
            if num == avg {
                result = false;
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
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::rearrange_array(nums);
        assert!(Solution::is_valid(result));
    }

    #[test]
    fn example_2() {
        let nums = vec![6, 2, 0, 9, 7];
        let result = Solution::rearrange_array(nums);
        assert!(Solution::is_valid(result));
    }
}
