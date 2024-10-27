struct Solution;

impl Solution {

    fn worker(nums: &Vec<i32>, start: usize, end: usize, count: usize) -> i32 {
        let mut result = i32::MAX;

        if count == 3 {
            result = nums[end] - nums[start];
        } else {
            result = result.min(Self::worker(nums, start+1, end, count+1));
            result = result.min(Self::worker(nums, start, end-1, count+1));
        }

        result
    }

    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let result;
        let n = nums.len();
        if n <= 4 {
            result = 0;
        } else {
            let mut nums = nums;
            nums.sort();
            result = Self::worker(&nums, 0, n-1, 0);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![5,3,2,4];
        let result = Solution::min_difference(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,5,0,10,14];
        let result = Solution::min_difference(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let nums = vec![3,100,20];
        let result = Solution::min_difference(nums);
        assert_eq!(result, 0);
    }

}
