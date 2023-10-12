/// Given an integer array `nums` of length `n` and an integer `target`, find three integers in
/// `nums` such that the sum is closest to `target`.
///
/// Return the sum of the three integers.
///
/// You may assume that each input would have exactly one solution.
struct Solution;

impl Solution {

    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let n = nums.len();
        let mut closest = -20000;

        let mut last_num1 = -20000;
        for i in 0..n-2 {
            let num1 = nums[i];
            if num1 != last_num1 {
                let mut last_num2 = -20000;
                for j in i+1..n-1 {
                    let num2 = nums[j];
                    if num2 != last_num2 {
                        let partial = num1 + num2;
                        let diff = target - partial;

                        let mut left = j + 1;
                        let mut right = n - 1;

                        while left <= right {
                            let mid = left + (right - left) / 2;
                            let third = nums[mid];
                            let total = partial + third;

                            if (target - total).abs() < (target - closest).abs() {
                                closest = total;
                            }

                            if third == diff {
                                break;
                            } else if third < diff {
                                left = mid + 1;
                            } else {
                                right = mid - 1;
                            }
                        }

                        if closest == target {
                            break;
                        }
                    }
                    last_num2 = num2;
                }

                if closest == target {
                    break;
                }
            }
            last_num1 = num1;
        }

        closest
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-1,2,1,-4];
        let target = 1;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,0,0];
        let target = 1;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums = vec![0,1,2];
        let target = 0;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 3);
    }

}

// 1. Write down the problem ✓
//
// 2. Clarify the problem space
// ** Input: nums: integer array
// ** Input: target: target sum
// ** Output: Closest sum to target
// ** nums.len >= 3 and <= 500
// ** nums element >= -1000 and <= 1000
// ** target >= -10_000 and <= 10_000
//
// 3. Write down the test cases
// ** Input: nums = [-1,2,1,-4], target = 1
// ** Output: 2
//
// ** Input: nums = [0,0,0], target = 1
// ** Output: 0
//
// 4. Describe and write down the algorithm
// ** First, sort the nums in place
// ** let n = nums.len()
// ** for i in 0..n-2
// **   for j in i+1..n-1
// **     binary search for closest sum from j+1 to n-1 inclusive.
// ** Potentially keep track whether i value or j value is equal to the last, if so, skip it.
// ** Time complexity: 
// **   Sorting is O(n log n) but that is dominated by the outer/inner for loops of O(n^2). Binary
// **   search component is O(log n), so it's O(n^2 * log n) or O(n^2) simplified.
// ** Space complexity: 
// **   Sorting is reusing the existing array for its output, so then it matters how much auxiliary
// **   space is required while sorting. Rust sort_unstable uses pattern defeating quicksort.
//
// 5. Code the algorithm
