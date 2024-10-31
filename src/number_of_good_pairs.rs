use std::collections::HashMap;

/// Given an array of integers `nums`, return the number of good pairs.
///
/// A pair `(i, j)` is called good if `nums[i] == nums[j]` and `i` < `j`.
struct Solution;

impl Solution {

    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            if counts.contains_key(&num) {
                result += counts[&num];
            }
            counts
                .entry(num)
                .and_modify(|c| { *c += 1 })
                .or_insert(1);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,1,1,3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,1,1,1];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2,3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 0);
    }


}
