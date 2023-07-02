#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Comparison {
    Unknown,
    Greater,
    Lesser,
}

/// Given an integer array `arr`, return the length of a maximum size turbulent subarray of `arr`.
///
/// A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in
/// the subarray.
///
/// More formally, a subarray `[arr[i], arr[i+1], ..., arr[j],]` of `arr` is said to be turbulent
/// if and only if:
///
/// * For `i <= k < j`:
///   * `arr[i] > arr[k+1]` when `k` is odd, and
///   * `arr[k] < arr[k+1]` when `k` is even.
/// * Or, for `i <= k < j`:
///   * `arr[k] > arr[k+1]` when `k` is even, and
///   * `arr[k] < arr[k+1]` when `k` is odd.
struct Solution;

impl Solution {

    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut max_length = 1;
        let mut current_length = 1;
        let mut last_value = arr[0];
        let mut comparison = Comparison::Unknown;

        for i in 1..n {
            let num = arr[i];
            if num == last_value {
                current_length = 1;
                comparison = Comparison::Unknown;
            } else if comparison == Comparison::Unknown {
                current_length += 1;
                if num > last_value {
                    comparison = Comparison::Lesser;
                } else {
                    comparison = Comparison::Greater;
                }
            } else if comparison == Comparison::Greater {
                if num > last_value {
                    current_length += 1;
                    comparison = Comparison::Lesser;
                } else {
                    current_length = 2;
                    comparison = Comparison::Greater;
                }
            } else {
                if num < last_value {
                    current_length += 1;
                    comparison = Comparison::Greater;
                } else {
                    current_length = 2;
                    comparison = Comparison::Lesser;
                }
            }
            last_value = num;
            max_length = max_length.max(current_length);
        }

        max_length
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let arr = vec![9,4,2,10,7,8,8,1,9];
        let result = Solution::max_turbulence_size(arr);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let arr = vec![4,8,12,16];
        let result = Solution::max_turbulence_size(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let arr = vec![100];
        let result = Solution::max_turbulence_size(arr);
        assert_eq!(result, 1);
    }

}
