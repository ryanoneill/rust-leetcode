/// Given an array of integers `nums` contains `n + 1` integers where each integer is in the range
/// `[1,n]` inclusive.
///
/// There is only one repeated number in `nums`, return this repeated number.
///
/// You must solve the problem without modifying the array `nums` and uses only constant extra
/// space.
struct Solution;

impl Solution {

    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut tortoise = nums[0];
        let mut hare = nums[0];

        loop {
            tortoise = nums[tortoise as usize];
            hare = nums[hare as usize];
            hare = nums[hare as usize];

            if tortoise == hare {
                break;
            }
        }

        tortoise = nums[0];
        while tortoise != hare {
            tortoise = nums[tortoise as usize];
            hare = nums[hare as usize];
        }
        hare
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,3,4,2,2];
        let result = Solution::find_duplicate(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,1,3,4,2];
        let result = Solution::find_duplicate(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn real_world() {
        let nums = vec![2,2,2,2,2];
        let result = Solution::find_duplicate(nums);
        assert_eq!(result, 2);
    }

}
