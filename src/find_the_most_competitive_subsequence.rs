/// Given an integer array `nums` and a positive integer `k`, return the most competitive
/// subsequence of `nums` of size `k`.
///
/// An array's subsequence is a resulting sequence obtained by erasing some (possibly zero)
/// elements from the array.
///
/// We define that a subsequence `a` is more competitive than a subsequence `b` (of the same
/// length) if in the first position where `a` and `b` differ, subsequence `a` has a number less
/// than the corresponding number in `b`. For example, `[1,3,4]` is more competitive than `[1,3,5]`
/// because the first position they differ is at the final number, and `4` is less than `5`.
struct Solution;

impl Solution {

    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut result = Vec::with_capacity(k);
        let mut stack = Vec::new();
        let mut left = n - k;

        for num in nums {
            while !stack.is_empty() {
                if left > 0 {
                    let last = *stack.last().unwrap();
                    if num < last {
                        stack.pop();
                        left -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            stack.push(num);
        }

        for i in 0..k {
            result.push(stack[i]);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,5,2,6];
        let k = 2;
        let result = Solution::most_competitive(nums, k);
        assert_eq!(result, vec![2,6]);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,4,3,3,5,4,9,6];
        let k = 4;
        let result = Solution::most_competitive(nums, k);
        assert_eq!(result, vec![2,3,3,4]);
    }

    #[test]
    fn example_3() {
        let nums = vec![5,4,3,2,1];
        let k = 3;
        let result = Solution::most_competitive(nums, k);
        assert_eq!(result, vec![3,2,1]);
    }

}
