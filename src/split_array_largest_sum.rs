struct Solution;

impl Solution {

    fn maximum(nums: &Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        for num in nums {
            result = result.max(*num);
        }
        result
    }

    fn sum(nums: &Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result += *num;
        }
        result
    }

    fn groupings(nums: &Vec<i32>, maximum: i32) -> i32 {
        let mut current = 0;
        let mut result = 0;

        for value in nums {
            let candidate = current + *value;
            if candidate <= maximum {
                current = candidate;
            } else {
                result += 1;
                current = *value;
            }
        }

        result + 1
    }

    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut left = Self::maximum(&nums);
        let mut right = Self::sum(&nums);

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::groupings(&nums, mid) <= k {
                right = mid - 1;
                result = mid;
            } else {
                left = mid + 1;
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
        let nums = vec![7,2,5,10,8];
        let k = 2;
        let result = Solution::split_array(nums, k);
        assert_eq!(result, 18);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,5];
        let k = 2;
        let result = Solution::split_array(nums, k);
        assert_eq!(result, 9);
    }

}
