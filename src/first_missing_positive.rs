/// Given an unsorted integer array `nums`, return the smallest missing positive integer.
///
/// You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.
struct Solution;

impl Solution {

    // nums.len() == n
    // which means nums should hold the values from 1 to n
    // index of 1 should be 0
    // index of 2 should be 1 and so on
    //
    // Uses Cycle Sort
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let top = n as i32;

        for i in 0..n {
            loop {
                let value = nums[i];
                if value <= 0 || value > top {
                    nums[i] = 0;
                    break;
                } else if value == (i+1) as i32 {
                    break;
                } else {
                    let index = value as usize - 1;
                    if nums[index] == value {
                        nums[i] = 0;
                        break;
                    } else {
                        nums.swap(i, index);
                    }
                }
            }
        }

        let mut result = 1;
        for i in 0..n {
            let num = nums[i];
            if num == result {
                result += 1;
            } else {
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
        let nums = vec![1,2,0];
        let result = Solution::first_missing_positive(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,4,-1,1];
        let result = Solution::first_missing_positive(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let nums = vec![7,8,9,11,12];
        let result = Solution::first_missing_positive(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let nums = vec![1,1];
        let result = Solution::first_missing_positive(nums);
        assert_eq!(result, 2);
    }

}
