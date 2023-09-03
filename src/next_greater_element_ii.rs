#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    index: usize,
    value: i32,
}

impl Item {

    pub fn new(index: usize, value: i32) -> Self {
        Self { index, value }
    }

}

/// Given a circular integer array `nums` (i.e., the next element of `nums[nums.length-1]` is
/// `nums[0]`), return the next greater number for every element in `nums`.
///
/// The next greater number of a number `x` is the first greater number to its traversing-order
/// next in the array, which means you could search circularly to find its next greater number. If
/// it doesn't exist, return `-1` for this number.
struct Solution;

impl Solution {

    fn find_largest_index(nums: &Vec<i32>) -> usize {
        let n = nums.len();

        let mut largest = i32::MIN;
        let mut result = n;

        for i in 0..n {
            let value = nums[i];
            if value > largest {
                largest = value;
                result = i;
            }
        }

        result
    }

    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];

        let mut stack = Vec::new();
        let largest_index = Self::find_largest_index(&nums);

        for i in 0..n {
            let j = (i + largest_index + 1) % n;
            let value = nums[j];
            let item = Item::new(j, value);

            while !stack.is_empty() {
                let last: Item = stack.last().copied().unwrap();
                if value > last.value {
                    stack.pop();
                    result[last.index] = value;
                } else {
                    break;
                }
            }
            stack.push(item);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,1];
        let result = Solution::next_greater_elements(nums);
        assert_eq!(result, vec![2,-1,2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,3];
        let result = Solution::next_greater_elements(nums);
        assert_eq!(result, vec![2,3,4,-1,4]);
    }

    #[test]
    fn example_3() {
        let nums = vec![-3,-2,-2,-3];
        let result = Solution::next_greater_elements(nums);
        assert_eq!(result, vec![-2,-1,-1,-2]);
    }

}
