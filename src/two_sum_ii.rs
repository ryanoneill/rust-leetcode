/// Given a 1-indexed array of integers `numbers` that is already sorted in non-decreasing order,
/// find two numbers such that they add up to a specific `target` number. Let these two numbers be
/// `numbers[index1]` and `numbers[index2]` where
/// `1 <= index1 < index2 < numbers.length`.
///
/// Return the indices of the two numbers, `index1` and `index2`, added by one
/// as an integer array `[index1, index2]` of length 2.
///
/// The tests are generated such that there is exactly one solution. You may
/// not use the same element twice.
///
/// Your solution must use only constant extra space.
struct Solution;

impl Solution {

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![0; 2];

        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                result[0] = left as i32 + 1;
                result[1] = right as i32 + 1;
                break;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
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
        let numbers = vec![2,7,11,15];
        let target = 9;
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn example_2() {
        let numbers = vec![2,3,4];
        let target = 6;
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, vec![1,3]);
    }

    #[test]
    fn example_3() {
        let numbers = vec![-1,0];
        let target = -1;
        let result = Solution::two_sum(numbers, target);
        assert_eq!(result, vec![1,2]);
    }

}
