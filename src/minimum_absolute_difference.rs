/// Given an array of distinct integers `arr`, find all
/// pairs of elements with the minimum absolute difference
/// of any two elements.
///
/// Return a list of pairs in ascending order (with repsect to pairs), each
/// pair `[a,b]` as follows
///
/// * `a, b` are from `arr`
///
/// * `a < b`
///
/// * `b - a` equals to the minimum absolute difference of any two elements in `arr`.
struct Solution;

impl Solution {

    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let mut result = Vec::new();
        let mut min_diff = i32::MAX;

        let mut a;
        let mut b;

        for pair in arr.windows(2) {
            a = pair[0];
            b = pair[1];

            let diff = b - a;
            if diff < min_diff {
                min_diff = diff;
                result.clear();
                result.push(vec![a, b]);
            } else if diff == min_diff {
                result.push(vec![a,b]);
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
        let arr = vec![4,2,1,3];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1,2], vec![2,3], vec![3,4]]);
    }

    #[test]
    fn example_2() {
        let arr = vec![1,3,6,10,15];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1,3]]);
    }

    #[test]
    fn example_3() {
        let arr = vec![3,8,-10,23,19,-4,-14,27];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![-14,-10], vec![19,23], vec![23,27]]);
    }

}
