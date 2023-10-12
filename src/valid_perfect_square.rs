/// Given an positive integer num, return `true` if `num` is a perfect square or `false` otherwise.
///
/// A perfect square is an integer that is the square of an integer. In other words, it is the
/// product of some integer with itself. 
///
/// You must not use any built-in library function, such as `sqrt`.
struct Solution;

impl Solution {

    // i32::MAX is 2147483647
    // sqrt of i32::MAX is 46340.95
    // 46340 * 46340 = 2147395600
    pub fn is_perfect_square(num: i32) -> bool {
        let mut result = false;

        if num <= 2147395600 {
            let mut left = 1;
            let mut right = 46340;

            while left <= right {
                let mid = left + (right - left) / 2;
                let square = mid * mid;
                if square == num {
                    result = true;
                    break;
                } else if square > num {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
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
        let num = 16;
        let result = Solution::is_perfect_square(num);
        assert!(result)
    }

    #[test]
    fn example_2() {
        let num = 14;
        let result = Solution::is_perfect_square(num);
        assert!(!result);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space
// ** Input: num: positive integer num
// ** Output: return `true` if num is a perfect square, false otherwise.
// ** num >= 1 and num <= i32::MAX.
//
// 3. Write down test cases
// ** Input: num = 16
// ** Output: true (4 x 4)
//
// ** Input: 14
// ** Output: false (3 x 3 = 9) (4 x 4 = 16)
//
// 4. Describe and write down the algorithm
// ** If num > 46340 squared, then false
// ** else do binary search where left = 1 and right = 46340
// ** If mid squared == num, result is true, otherwise iterate until left no longer <= right
//
// 5. Code the algorithm
