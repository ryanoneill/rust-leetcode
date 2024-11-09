/// Given an array `nums` of length `n`, return an array `answer` of length `n-1` such that
/// `answer[i] = nums[i] | nums[i+1]` where `|` is the bitwise `OR` operation.
struct Solution;

impl Solution {

    pub fn or_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut previous = nums[0];

        let mut results = Vec::with_capacity(n-1);

        for i in 1..n {
            let current = nums[i];
            let result = previous | current;
            results.push(result);
            previous = current;
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,3,7,15];
        let result = Solution::or_array(nums);
        assert_eq!(result, vec![3,7,15]);
    }

    #[test]
    fn example_2() {
        let nums = vec![8,4,2];
        let result = Solution::or_array(nums);
        assert_eq!(result, vec![12,6]);
    }

    #[test]
    fn example_3() {
        let nums = vec![5,4,9,11];
        let result = Solution::or_array(nums);
        assert_eq!(result, vec![5,13,11]);
    }

}
