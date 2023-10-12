/// Given a non-negative integer `x`, return the square root of `x` rounded down to the nearest
/// integer. The returned integer should be non-negative as well.
///
/// You must not use any built-in exponent function or operator.
struct Solution;

impl Solution {

    // 46340 * 46340 = 2147395600
    pub fn my_sqrt(x: i32) -> i32 {
        let result;
        if x == 0 {
            result = 0;
        } else if x >= 2147395600 {
            result = 46340;
        } else {
            let mut left = 1;
            let mut right = 46340;

            while left <= right {
                let mid = left + (right - left) / 2;
                let squared = mid * mid;
                if squared == x {
                    right = mid;
                    break;
                } else if squared > x {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            result = right;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let x = 4;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let x = 8;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let x = 0;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_4() {
        let x = i32::MAX;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 46340);
    }

}

// 1. Write down the problem ✓
//
// 2. Clarify the problem space ✓
// ** Input: x, non-negative integer
// ** Output: non-negative integer, square root of 'x', rounded down to the nearest integer. 
// ** x >= 0 and x <= i32::MAX
// 
// 3. Write down the test cases ✓
// ** Input: x = 4
// ** Output: 2
//
// ** Input: 8
// ** Output 2
//
// ** Input: 0
// ** Output 0
//
// ** Input: i32::MAX
// ** Output: 46340
//
// 4. Describe and write down the algorithm
// ** If x is 0, then the result is 0
// ** If x is greater than or equal to 46340 squared, then the result is 46340
// ** else binary search the transition point starting with left = 1 and right = 46340
