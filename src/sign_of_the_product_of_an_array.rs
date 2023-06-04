#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Sign {
    Positive,
    Negative,
    Zero,
}

/// There is a function `signFunc(x)` that returns:
///
/// * `1` if `x` is positive.
///
/// * `-1` if `x` is negative.
///
/// * `0` if `x` is equal to `0`.
///
/// You are given an integer array `nums`. Let `product` be the product of all values in the array
/// `nums`.
///
/// Return `signFunc(product)`.
struct Solution;

impl Solution {

    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut result = Sign::Positive;

        for num in nums {
            match num {
                0 => {
                    result = Sign::Zero;
                    break;
                }
                x if x < 0 => {
                    if result == Sign::Positive {
                        result = Sign::Negative;
                    } else if result == Sign::Negative {
                        result = Sign::Positive;
                    }
                }
                _ => { } // skip, positive doesn't change anything
            }
        }

        if result == Sign::Zero {
            0
        } else if result == Sign::Positive {
            1
        } else {
            -1
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-1,-2,-3,-4,3,2,1];
        let result = Solution::array_sign(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,5,0,2,-3];
        let result = Solution::array_sign(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums = vec![-1,1,-1,1,-1];
        let result = Solution::array_sign(nums);
        assert_eq!(result, -1);
    }

}
