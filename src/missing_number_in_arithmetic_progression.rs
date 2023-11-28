/// In some array `arr`, the values were in arithmetic progression: the values `arr[i+1] - arr[i]`
/// are all equal for every `0 <= i < arr.length - 1`.
///
/// A value from `arr` was removed that was not the first or last value in the array.
///
/// Given `arr`, return the removed value.
struct Solution; 

impl Solution {

    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let mut result = i32::MIN;

        let first = arr[0];
        let second = arr[1];
        let third = arr[2];

        let diff_one = second - first;
        let diff_two = third - second;

        let expected = if diff_one > 0 {
            diff_one.min(diff_two)
        } else {
            diff_one.max(diff_two)
        };

        if expected == 0 {
            result = arr[0];
        } else {
            for pair in arr.windows(2) {
                let diff = pair[1] - pair[0];
                if diff != expected {
                    result = pair[0] + expected;
                    break;
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
        let arr = vec![5,7,11,13];
        let result = Solution::missing_number(arr);
        assert_eq!(result, 9);
    }

    #[test]
    fn example_2() {
        let arr = vec![15,13,12];
        let result = Solution::missing_number(arr);
        assert_eq!(result, 14);
    }

    #[test]
    fn example_3() {
        let arr = vec![0,0,0,0,0];
        let result = Solution::missing_number(arr);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_4() {
        let arr = vec![1,1,1,1,1,1];
        let result = Solution::missing_number(arr);
        assert_eq!(result, 1);
    }

}
