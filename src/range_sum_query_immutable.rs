/// Given an integer array `nums`, handle multiple queries of the following
/// type:
///
/// Calculate the sum of the elements of `nums` between indices `left` and
/// `right` inclusive where `left <= right`.
///
/// Implement the `NumArray` class:
/// * `NumArray(int[] nums)` Initializes the object with the integer array
///   `nums`.
/// * `int sumRange(int left, int right) Returns the sum of the elements of
///   `nums` between indices `left` and `right` inclusive (i.e.
///   `nums[left] + nums[left + 1] + ... + nums[right]`).
struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {

    fn prefix_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter().map(|x| { sum += x; sum }).collect()
    }

    fn new(nums: Vec<i32>) -> Self {
        Self {
            sums: Self::prefix_sum(nums)
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.sums[right as usize]
        } else {
            self.sums[right as usize] - self.sums[(left-1) as usize]
        }
    }

}

#[cfg(test)]
mod tests {
    use super::NumArray;

    #[test]
    fn example_1() {
        let num_array = NumArray::new(vec![-2,0,3,-5,2,-1]);
        let result1 = num_array.sum_range(0, 2);
        assert_eq!(result1, 1);
        let result2 = num_array.sum_range(2, 5);
        assert_eq!(result2, -1);
        let result3 = num_array.sum_range(0, 5);
        assert_eq!(result3, -3);
    }

}
