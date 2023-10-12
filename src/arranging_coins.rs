/// You have `n` coins and you want to build a staircase with these coins. The staircase consists
/// of `k` rows where the `ith` row has exactly `i` coins. The last row of the staircase may be
/// incomplete.
///
/// Given the integer `n`, return the number of complete rows of the staircase you will build.
struct Solution;

impl Solution {

    pub fn arrange_coins(n: i32) -> i32 {
        let mut result = 0;
        if n >= 2147450880 {
            result = 65535;
        } else {
            let mut left = 1;
            let mut right = 65535;

            while left <= right {
                let mid = left + (right - left) / 2;
                let sum = if mid % 2 == 0 {
                    (mid / 2) * (mid + 1)
                } else {
                    mid * ((mid + 1) / 2)
                };
                if sum == n {
                    result = mid;
                    break;
                } else if sum > n {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            if result == 0 {
                result = right;
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
        let n = 5;
        let result = Solution::arrange_coins(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 8;
        let result = Solution::arrange_coins(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let n = 1804289383;
        let result = Solution::arrange_coins(n);
        assert_eq!(result, 60070);
    }

}

// 1. Write down the problem
//
// 2. Clarify the problem space
// ** Input: n: number of coins
// ** Output: integer number of complete rows of the staircase
// ** n >= 1 and n <= i32::MAX
//
// 3. Write down the test cases
// ** Input: n = 5
// ** Output: 2
//
// ** Input: n = 8
// ** Output: 3
//
// ** Input: n = 1
// ** Output: 1
//
// 4. Describe and write down the algorithm
// ** sum of numbers from 1 to m is (m * (m + 1)) / 2
// ** sum of 1 to 8 = (8 * (9)) / 2 = 36
// ** n = (m * (m + 1)) / 2
// ** 2n = m^2 + m 
// 2_000_000
