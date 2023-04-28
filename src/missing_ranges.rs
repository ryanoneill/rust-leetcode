/// You are given an inclusive range `[lower, upper]` and a sorted unique array
/// `nums`, where all elements are within the inclusive range.
///
/// A number `x` is considered missing if `x` is in the range `[lower, upper]`
/// and `x` is not in `nums`.
///
/// Return the shortest sorted list of ranges that exactly covers all the
/// missing numbers. That is, no element of `nums` is included in any of the
/// ranges, and each missing number is covered by one of the ranges.
struct Solution;

impl Solution {

    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut last_num = None;

        if nums.len() == 0 {
            result.push(vec![lower, upper]);
        } else {
            for num in nums {
                match last_num {
                    None => {
                        if num > lower {
                            result.push(vec![lower, num-1]);
                        }
                    }
                    Some(ln) => {
                        if ln != num - 1 {
                            result.push(vec![ln+1, num-1]);
                        }
                    }
                }
                last_num = Some(num);
            }
            match last_num {
                Some(ln) => {
                    if ln < upper {
                        result.push(vec![ln+1, upper]);
                    }
                }
                None => { }
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
        let nums = vec![0,1,3,50,75];
        let lower = 0;
        let upper = 99;
        let result = Solution::find_missing_ranges(nums, lower, upper);
        assert_eq!(result, vec![vec![2,2], vec![4,49],vec![51,74],vec![76,99]]);
    }

    #[test]
    fn example_2() {
        let nums = vec![-1];
        let lower = -1;
        let upper = -1;
        let result = Solution::find_missing_ranges(nums, lower, upper);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn empty_nums() {
        let nums = vec![];
        let lower = 10;
        let upper = 90;
        let result = Solution::find_missing_ranges(nums, lower, upper);
        assert_eq!(result, vec![vec![10,90]]);
    }

    #[test]
    fn negative() {
        let nums = vec![-23, -15, -10];
        let lower = -30;
        let upper = -2;
        let result = Solution::find_missing_ranges(nums, lower, upper);
        assert_eq!(result, vec![vec![-30,-24], vec![-22,-16], vec![-14,-11], vec![-9,-2]]);
    }

}

