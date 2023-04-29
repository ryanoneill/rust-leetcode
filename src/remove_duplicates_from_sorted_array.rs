/// Given an integer array `nums` sorted in non-decreasing order, remove the
/// duplicates in-place such that each unique element appears only once. The
/// relative order of the elements should be kept the same. Then return the
/// number of unique elements in `nums`.
///
/// Consider the number of unique elements of `nums` to be `k`, to get
/// accepted, you need to do the following things:
/// * Change the array `nums` such that the first `k` elements of `nums`
///   contain the unique elements in the order they were present in `nums`
///   initially. The remaining elements of `nums` are not important as well
///   as the size of `nums`.
/// * Return `k`.
struct Solution;

impl Solution {

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { 0 }
        else {
            let mut last = nums[0];
            let mut i = 1;

            for j in 1..n {
                let num = nums[j];
                if num != last {
                    last = num;
                    nums[i] = num;
                    i += 1;
                }
            }

            i as i32
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1,1,2];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        let _ = nums.split_off(2);
        assert_eq!(nums, vec![1,2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        let _ = nums.split_off(5);
        assert_eq!(nums, vec![0,1,2,3,4]);
    }

}
