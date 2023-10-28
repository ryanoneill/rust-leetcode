/// You are given `m` `arrays`, where each array is sorted in ascending order.
///
/// You can pick up two integers from two different arrays (each array picks one) and calculate the
/// distance. We define the distance between two integers `a` and `b` to be their absolute
/// difference `|a - b|`.
///
/// Return the maximum distance.
struct Solution;

impl Solution {

    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let m = arrays.len();
        let n = arrays[0].len();

        let mut previous_min = arrays[0][0];
        let mut previous_max = arrays[0][n-1];

        let mut result = i32::MIN;

        for i in 1..m {
            let k = arrays[i].len();
            let current_min = arrays[i][0];
            let current_max = arrays[i][k-1];

            let diff_one = previous_max - current_min;
            let diff_two = current_max - previous_min;
            let diff = diff_one.max(diff_two);
            result = result.max(diff);

            previous_min = previous_min.min(current_min);
            previous_max = previous_max.max(current_max);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let arrays = vec![vec![1,2,3], vec![4,5], vec![1,2,3]];
        let result = Solution::max_distance(arrays);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let arrays = vec![vec![1], vec![1]];
        let result = Solution::max_distance(arrays);
        assert_eq!(result, 0);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space
// ** Input: arrays = array of array of ints (Vec<Vec<i32>>)
// ** Output: int (i32)
//
// ** m is between 2 and 100_000 (number of arrays)
// ** n is between 1 and 500 (length of each array)
//
// Values must be from different arrays.
//
// 3. Write down test cases ✓
// ** Input: arrays = [[1,2,3], [4,5], [1,2,3]]
// ** Output: 4 (5 - 1)
//
// ** Input: arrays = [[1],[1]]
// ** Output: 0
//
// 4. Describe and write down the algorithm ✓
// ** Take min and max of first array (x[0] and x[n-1])
// ** Take min and max of next array, calculate max - previous min and previous max - min, and
//    keep track of the max difference. Store min of all arrays as min, and max of all arrays as
//    max.
// ** Keep going until all arrays have been visited.
//
// 5. Start coding
