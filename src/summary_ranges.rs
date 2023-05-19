/// You are given a sorted unique integer array `nums`.
///
/// A range `[a, b]` is the set of integers from `a` to `b` (inclusive).
///
/// Return the smallest sorted list of ranges that cover all the numbers
/// in the array exactly. That is, each element of `nums` is covered by
/// exactly one of the ranges, and there is no integer `x` such that `x` is in
/// one of the ranges but not in `nums`.
///
/// Each range `[a,b]` in the list should be output as:
///
/// * `"a->b"` if `a != b`
///
/// * `"a"` if `a == b`
struct Solution;

impl Solution {

    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        if n == 0 { vec![] }
        else {
            let mut result = Vec::new();
            let mut start = nums[0];
            let mut end = nums[0];

            for i in 1..n {
                let value = nums[i];
                if end + 1 == value {
                    end = value;
                } else if start == end {
                    result.push(start.to_string());
                    start = value;
                    end = value;
                } else {
                    let s = start.to_string() + "->" + &end.to_string();
                    result.push(s);
                    start = value;
                    end = value;
                }
            }

            if start == end {
                result.push(start.to_string());
            } else {
                let s = start.to_string() + "->" + &end.to_string();
                result.push(s);
            }

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![0,1,2,4,5,7];
        let result = Solution::summary_ranges(nums);
        assert_eq!(result, vec!["0->2", "4->5", "7"]);
    }

    #[test]
    fn example_2() {
        let nums = vec![0,2,3,4,6,8,9];
        let result = Solution::summary_ranges(nums);
        assert_eq!(result, vec!["0", "2->4", "6", "8->9"]);
    }

}
