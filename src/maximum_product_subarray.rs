/// Given an integer array `nums`, find a subarray that has the largest product, and return the
/// product.
///
/// The test cases are generated so that the answer will fit in a 32-bit integer.
struct Solution;

impl Solution {

    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;

        let mut negative_result = i32::MAX;
        let mut positive_result = i32::MIN;

        for num in nums {
            if num == 0 {
                result = result.max(0);
                negative_result = i32::MAX;
                positive_result = i32::MIN;
            } else if num > 0 {
                if negative_result != i32::MAX {
                    negative_result *= num;
                }
                if positive_result == i32::MIN {
                    positive_result = num;
                } else {
                    positive_result *= num;
                }
                result = result.max(positive_result);
            } else { // num < 0
                if negative_result == i32::MAX {
                    if positive_result != i32::MIN {
                        negative_result = positive_result * num;
                        positive_result = i32::MIN;
                    } else {
                        negative_result = num;
                    }
                    result = result.max(num);
                } else {
                    let temp = negative_result * num;
                    negative_result = if positive_result != i32::MIN {
                        num * positive_result
                    } else {
                        num
                    };
                    positive_result = temp;
                    result = result.max(positive_result);
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
        let nums = vec![2,3,-2,4];
        let result = Solution::max_product(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![-2,0,-1];
        let result = Solution::max_product(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let nums = vec![-2];
        let result = Solution::max_product(nums);
        assert_eq!(result, -2);
    }

    #[test]
    fn example_4() {
        let nums = vec![7,-2,-4];
        let result = Solution::max_product(nums);
        assert_eq!(result, 56);
    }

    #[test]
    fn example_5() {
        let nums = vec![-1,-2,-9,-6];
        let result = Solution::max_product(nums);
        assert_eq!(result, 108);
    }

}
