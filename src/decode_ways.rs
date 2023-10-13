use std::collections::HashMap;

/// A message containing letters from `A-Z` can be encoded into numbers using the following
/// mapping:
/// 'A' -> "1"
/// 'B' -> "2"
/// ...
/// 'Z' -> "26"
///
/// To decode an encoded message, all the digits must be grouped then mapped back into letters
/// using the reverse of the mapping above (there may be multiple ways). For example `"11106"` can
/// be mapped into:
///
/// * `"AAJF"` with the grouping `(1 1 10 6)`
/// * `"KJF"` with the grouping `(11 10 6)`
///
/// Note that the grouping `(1 11 06)` is invalid because `"06"` cannot be mapped into `'F'` since
/// `"6"` is different from `"06"`.
///
/// Given a string `s` containing only digits, return the number of ways to decode it.
///
/// The test cases are generated so that the answer fits in a 32-bit integer.
struct Solution;

impl Solution {

    fn char_to_digit(c: char) -> i32 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _   => 0,
        }
    }

    fn worker(results: &mut HashMap<usize, i32>, nums: &Vec<i32>, index: usize) -> i32 {
        let result;
        let n = nums.len();
        if index == n {
            result = 1;
        } else if results.contains_key(&index) {
            result = results[&index];
        } else {
            let value = nums[index];
            if value == 0 {
                result = 0;
            } else if value == 1 {
                if index == n-1 {
                    result = 1;
                } else {
                    let next = nums[index+1];
                    if next == 0 {
                        result = Self::worker(results, nums, index+2);
                    } else {
                        result = Self::worker(results, nums, index+1) + 
                            Self::worker(results, nums, index+2);
                    }
                }
            } else if value == 2 {
                if index == n-1 {
                    result = 1;
                } else {
                    let next = nums[index+1];
                    if next == 0 {
                        result = Self::worker(results, nums, index+2);
                    } else if next > 6 {
                        result = Self::worker(results, nums, index+1);
                    } else {
                        result = Self::worker(results, nums, index+1) +
                            Self::worker(results, nums, index+2);
                    }
                }
            } else {
                result = Self::worker(results, nums, index+1);
            }

            results.insert(index, result);
        }
        result
    }

    pub fn num_decodings(s: String) -> i32 {
        let nums: Vec<i32> = s.chars().map(Self::char_to_digit).collect();
        let mut results = HashMap::new();
        Self::worker(&mut results, &nums, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "12".to_string();
        let result = Solution::num_decodings(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let s = "226".to_string();
        let result = Solution::num_decodings(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let s = "06".to_string();
        let result = Solution::num_decodings(s);
        assert_eq!(result, 0);
    }

}


// 1. Write down the problem ✓
//
// 2. Clarify the problem space
// ** Input: s: String of numbers
// ** Output: integer: number of ways to decode the string.
// s.len() >= 1 and <= 100
// s contains only digits and may contain leading zero(s).
//
// 3. Write down the test cases
// ** Input: s = "12"
// ** Output: 2 ("12" could be "AB" or "L")
//
// ** Input: s = "226"
// ** Output: 3 ("226" could be "BBF", "BZ", or "VF")
//
// ** Input: s = "06"
// ** Output: 0
//
// 4. Describe and write down the algorithm
// Convert s to Vec<i32> 
// Start with index = 0
// Dynamic programming problem with overlapping subproblems.
// If the current index only has one option, then the result is the result of subproblem.
// If it has more than one option, then the result is the combination of its subproblems.
// Time complexity: O(n)
// Space complexity: O(n)
//
//
