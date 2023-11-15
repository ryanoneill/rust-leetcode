/// Given an array `nums` of size `n`, return the majority element.
///
/// The majority element is the element that appears more than `⌊n / 2⌋` times.
/// You may assume that the majority element always exists in the array.
struct Solution;

impl Solution {

    // Implementation changed to use Boyer-Moore
    // O(N) time
    // O(1) space
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority: i32 = i32::MIN;
        let mut counter: i32 = 0;

        for num in nums {
            if counter == 0 {
                majority = num;
                counter = 1;
            } else if num == majority {
                counter += 1;
            } else {
                counter -= 1;
            }
        }

        majority
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3, 2, 3];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 2);
    }
}
