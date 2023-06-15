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

    // TODO: Implement
    pub fn find_min(_nums: Vec<i32>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let nums = vec![3,4,5,1,2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 1);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let nums = vec![4,5,6,7,0,1,2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 0);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let nums = vec![11,13,15,17];
        let result = Solution::find_min(nums);
        assert_eq!(result, 11);
    }

}
