/// Given an integer array of size `n`, find all elements that appear more than `[n/3]` times.
struct Solution;

impl Solution {

    // Implementation changed to use Boyer-Moore
    // O(N) time
    // O(1) space
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut candidate1 = i32::MIN;
        let mut candidate2 = i32::MIN;
        let mut count1 = 0;
        let mut count2 = 0;

        for &num in &nums {
            if candidate1 == num {
                count1 += 1;
            } else if candidate2 == num {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = num;
                count1 += 1;
            } else if count2 == 0 {
                candidate2 = num;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        count1 = 0;
        count2 = 0;

        for &num in &nums {
            if candidate1 == num {
                count1 += 1;
            } else if candidate2 == num {
                count2 += 1;
            }
        }

        let n = nums.len();
        let min_count = n / 3;

        let mut result = Vec::new();
        if count1 > min_count {
            result.push(candidate1);
        }
        if count2 > min_count {
            result.push(candidate2);
        }
        
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,2,3];
        let result = Solution::majority_element(nums);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let result = Solution::majority_element(nums);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2];
        let result = Solution::majority_element(nums);
        assert_eq!(result, vec![1,2]);
    }

}
