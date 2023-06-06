/// A sequence of numbers is called an arithmetic progression if the difference between any two
/// consecutive elements is the same.
///
/// Given an array of numbers `arr`, return `true` if the array can be rearranged to form an
/// arithmetic progression. Otherwise, return `false`.
struct Solution;

impl Solution {

    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        let mut result = true;
        let diff = arr[1] - arr[0];
        for i in 2..arr.len() {
            if arr[i] - arr[i-1] != diff {
                result = false;
                break;
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
        let arr = vec![3,5,1];
        let result = Solution::can_make_arithmetic_progression(arr);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let arr = vec![1,2,4];
        let result = Solution::can_make_arithmetic_progression(arr);
        assert!(!result);
    }

}
