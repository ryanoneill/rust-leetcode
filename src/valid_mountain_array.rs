/// Given an array of integers `arr`, return `true` if and only if it is a valid mountain array.
///
/// Recall that arr is a mountain array if and only if:
/// * arr.length >= 3
///
/// * There exists some `i` with `0 < i < arr.length - 1` such that:
///   * `arr[0] < arr[1] < ... < arr[i-1] < arr[i]`
///   * `arr[i] > arr[i+1] > ... arr[arr.length - 1]`
struct Solution;

impl Solution {

    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut result = false;
        let n = arr.len();

        if n >= 3 {
            let mut last = arr[0];
            let mut increasing = true;

            result = true;

            for i in 1..n {
                let num = arr[i];
                if num == last {
                    result = false;
                    break;
                } else if increasing {
                    if num < last {
                        if i == 1 {
                            result = false;
                            break;
                        } else {
                            increasing = false;
                        }
                    }
                } else if num > last {
                    result = false;
                    break;
                }
                last = num;
            }

            if increasing {
                result = false;
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
        let arr = vec![2,1];
        let result = Solution::valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let arr = vec![3,5,5];
        let result = Solution::valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let arr = vec![0,3,2,1];
        let result = Solution::valid_mountain_array(arr);
        assert!(result);
    }

    #[test]
    fn example_4() {
        let arr = vec![1,2,3,4,5];
        let result = Solution::valid_mountain_array(arr);
        assert!(!result);
    }

    #[test]
    fn example_5() {
        let arr = vec![5,4,3,2,1];
        let result = Solution::valid_mountain_array(arr);
        assert!(!result);
    }

}

// 1. Write down the problem ✓
//
// 2. Clarify the problem space ✓
// ** Input: arr: array of integers
// ** Output: bool: true if it is a valid mountain array, false otherwise.
// ** arr.len() >= 1 and <= 10_000
// ** arr element >= 0 and <= 10_000
// 
// 3. Write down the test cases
// ** Input: arr = [2,1]
// ** Output: false
//
// ** Input: arr = [3,5,5]
// ** Output: false
//
// ** Input: arr = [0,3,2,1]
// ** Output: true
//
// 4. Describe and write down the algorithm
// ** Let n = arr.len()
// ** First check that n >= 3. If not, then false
// ** Need to check each element, so O(n)
// ** Keep track of whether increasing or descreasing.
// ** Set last to arr[0]
// ** Iterate through all elements starting at index 1.
// ** If element == last, result is false and break.
// ** If increasing and element is greater than last, keep going.
// ** If increasing and element is less than last, switch to decreasing and keep going.
// ** If decreasing and element is not less than last, result is false and break.
// ** If only ever increasing, then result is false.
// ** If only ever decreasing, then result is false.
// ** 
// ** Time complexity: O(n)
// ** Space complexity: O(1)
