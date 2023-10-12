/// An array `arr` is a mountain if the following properties hold:
/// * `arr.length >= 3`
/// * There exists some `i` with `0 < i < arr.length - 1` such that:
///   * `arr[0] < arr[1] < ... < arr[i-1] < arr[i]`
///   * `arr[i] > arr[i+1] > ... > arr[arr.length - 1]`
///
/// Given a mountain array `arr`, return the index `i` such that `arr[0] < arr[1] < ... arr[i-1] <
/// arr[i] > arr[i+1] > ... arr[arr.length-1]`.
///
/// You must solve it in O(log(arr.length)) time complexity.
struct Solution;

impl Solution {

    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut result = 0;

        let n = arr.len();

        let mut left = 1;
        let mut right = n-2; // n is guaranteed to be 3 or greater

        while left <= right {
            let mid = left + (right - left) / 2;
            let value = arr[mid];
            let previous = arr[mid-1];
            let next = arr[mid+1];

            if value > previous && value > next {
                result = mid as i32;
                break;
            } else if value > previous {
                left = mid + 1;
            } else {
                right = mid - 1;
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
        let arr = vec![0,1,0];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let arr = vec![0,2,1,0];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let arr = vec![0,10,5,2];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let arr = vec![1,2,3,4,5,6,7,6];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 6);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space
// ** Input: arr: mountain array of integers
// ** Output: integer index of the peak element
//
// 3. Test cases
// ** Input: [0,1,0]
// ** Output: 1
//
// ** Input: [0,2,1,0]
// ** Output: 1
//
// ** Input: [0,10,5,2]
// ** Output: 1
//
// 4. Describe and write down the algorithm
// ** Since arr is a mountain array, len must be >= 3.
// ** element 0 will not be the peak element.
// ** element n-1 will not be the peak element.
// ** binary search for the peak element with left = 1 and right = n-2
// ** if the middle element of 3 is greater than previous and greater than next, then that index is
// the peak.
// ** otherwise if the middle element of 3 is greater than previous and less than next, then the
// peak is to the right.
// ** otherwise the peak is to the left.
