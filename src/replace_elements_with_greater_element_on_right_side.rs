/// Given an array `arr`, replace every element in that array with the greatest element among the
/// elements to its right, and replace the last element with `-1`.
///
/// After doing so, return the array.
struct Solution;

impl Solution {

    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut results = vec![0; n];
        results[n-1] = -1;

        let mut max = i32::MIN;

        for i in (1..n).rev() {
            let value = arr[i];
            max = max.max(value);
            results[i-1] = max;
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let arr = vec![17,18,5,4,6,1];
        let result = Solution::replace_elements(arr);
        assert_eq!(result, vec![18,6,6,6,1,-1]);
    }

    #[test]
    fn example_2() {
        let arr = vec![400];
        let result = Solution::replace_elements(arr);
        assert_eq!(result, vec![-1]);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space ✓
// ** Input: arr: Array of integer elements
// ** Output: Array of integer elements
// ** Vec<i32> -> Vec<i32>
// ** Replace every element with the greatest element among the elemnts to its right
// ** Replace the last element with -1.
// ** arr.length >= 1 and <= 10_000
// ** arr element is >= 1 and <= 100_000
// ** 100_000 can be easily held in an i32. (Could use u32, but LeetCode uses i32)
//
// 3. Write down the test cases. ✓
// Input: [1,2,3,4,5]
// Output: [2,3,4,5,-1]
//
// Input: [5,4,3,2,1]
// Output: [4,3,2,1,-1]
//
// Input: [100]
// Output: [-1]
//
// Input: [17,18,5,4,6,1]
// Output: [18,6,6,6,1,-1]
//
// 4. Describe and Write Down the Algorithm ✓
// ** Allocate result array the size of `n`.
// ** Put a -1 in the last slot of the result array.
// ** For n-1..0 iterate through and keep track of the max and put that value in the next highest
// slot in the result array.
//
// 5. Start coding
