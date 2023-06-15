/// Suppose an array of length `n` sorted in ascending order is rotated between `1` and `n` times.
/// For example, the array `nums = [0,1,2,4,5,6,7]` might become:
///
/// * `[4,5,6,7,0,1,2]` if it was rotated `4` times.
///
/// * `[0,1,2,4,5,6,7]` if it was rotated `7` times.
///
/// Notice that rotating the array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time results in the array
/// `[a[n-1], a[0], a[1], a[2], ..., [an-2]]`.
///
/// Given the sorted rotated array `nums` of unique elements, return the minimum element of this
/// array.
///
/// You must write an algorithm that runs in `O(log n) time`.
struct Solution;

impl Solution {

    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        if nums.len() == 1 {
            result = nums[0];
        } else {
            let mut start = 0;
            let mut end = nums.len() - 1;
            if nums[start] < nums[end] {
                result = nums[start];
            } else {
                while start < end {
                    let mid = start + (end - start) / 2;
                    let mid_plus_one = mid + 1;
                    let mid_minus_one = mid - 1;

                    if nums[mid] > nums[mid_plus_one] {
                        result = nums[mid_plus_one];
                        break;
                    } else if nums[mid] < nums[mid_minus_one] {
                        result = nums[mid];
                        break;
                    } else if nums[mid] > nums[0] {
                        start = mid + 1;
                    } else {
                        end = mid - 1;
                    }
                }
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
        let nums = vec![3,4,5,1,2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let nums = vec![4,5,6,7,0,1,2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums = vec![11,13,15,17];
        let result = Solution::find_min(nums);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_4() {
        let nums = vec![3,1,2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 1);
    }

}
