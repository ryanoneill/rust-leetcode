use std::collections::HashMap;

/// Given an array of integers `arr`, a lucky integer is an integer that has a frequency in the
/// array equal to its value.
///
/// Return the largest lucky integer in the array. If there is no lucky integer return `-1`.
struct Solution;

impl Solution {

    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut counts = HashMap::new();

        for num in arr {
            counts
                .entry(num)
                .and_modify(|c| { *c += 1; })
                .or_insert(1);
        }

        for (key, value) in counts {
            if key == value {
                result = result.max(key);
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
        let arr = vec![2,2,3,4];
        let result = Solution::find_lucky(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let arr = vec![1,2,2,3,3,3];
        let result = Solution::find_lucky(arr);
        assert_eq!(result, 3);
    }

}
