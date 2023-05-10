/// Given an integer array `nums`, return `true` if there exists
/// a triple of indices `(i, j, k)` such that `i < j < k` and
/// `nums[i] < nums[j] < nums[k]`. If no such indices
/// exists, return `false`.
struct Solution;

impl Solution {

    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut i = i32::MAX;
        let mut j = i32::MAX;
        let mut result = false;

        for num in nums {
            if num <= i {
                i = num;
            } else if num <= j {
                j = num;
            } else {
                result = true;
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
        let nums = vec![1,2,3,4,5];
        let result = Solution::increasing_triplet(nums);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![5,4,3,2,1];
        let result = Solution::increasing_triplet(nums);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let nums = vec![2,1,5,0,4,6];
        let result = Solution::increasing_triplet(nums);
        assert!(result);
    }

}
