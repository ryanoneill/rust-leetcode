/// Given a binary array `nums`, return the maximum number of consecutive `1`'s in the array.'
struct Solution;

impl Solution {

    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut result = 0;

        for num in nums {
            if num == 1 {
                current += 1;
                result = result.max(current);
            } else {
                current = 0;
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
        let nums = vec![1,1,0,1,1,1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,0,1,1,0,1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn single_one() {
        let nums = vec![1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(result, 1);
    }

}
