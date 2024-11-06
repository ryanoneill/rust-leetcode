/// You are given a 0-indexed array of positive integers `nums`.
///
/// In one operation, you can swap any two adjacent elements if they have the same number of set
/// bits. You are allowed to do this operation any number of times (including zero).
///
/// Return `true` if you can sort the array, else return `false`.
struct Solution;

impl Solution {

    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut result = true;
        let n = nums.len();

        let mut previous_max = i32::MIN;

        let mut current_max = nums[0];
        let mut current_min = nums[0];

        let mut current_bits = nums[0].count_ones();

        for i in 0..n {
            let num = nums[i];
            let bits = num.count_ones();

            if bits == current_bits {
                current_max = current_max.max(num);
                current_min = current_min.min(num);
            } else if current_min < previous_max {
                result = false;
                break;
            } else {
                previous_max = current_max;

                current_max = num;
                current_min = num;
                current_bits = bits;
            }
        }

        if result {
            result = current_min >= previous_max;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![8,4,2,30,15];
        let result = Solution::can_sort_array(nums);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,5];
        let result = Solution::can_sort_array(nums);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let nums = vec![3,16,8,4,2];
        let result = Solution::can_sort_array(nums);
        assert!(!result);
    }

}
