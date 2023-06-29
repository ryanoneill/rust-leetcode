struct Solution;

impl Solution {

    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        for i in 0..=n {
            result ^= i as i32;
        }

        for num in nums {
            result ^= num;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,0,1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn non_two() {
        let nums = vec![5,3,1,2,0];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 4);
    }

}
