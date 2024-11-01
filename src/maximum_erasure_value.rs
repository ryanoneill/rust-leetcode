use std::collections::HashSet;

/// You are given an array of positive integers `nums` and want to erase a subarray containing
/// unique elements. The score you get by erasing the subarray is equal to the sum of its elements.
/// 
/// Return the maximum score you can get by erasing exactly one subarray.
///
/// An array `b` is called to be a subarray of `a` if it forms a contiguous subsequence of `a`,
/// that is, if it is equal to `a[l],a[l+1],...,a[r]` for some `(l,r)`.
struct Solution;

impl Solution {

    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut result = 0;
        let mut current = 0;
        let mut seen: HashSet<i32> = HashSet::new();

        let mut left = 0;
        for right in 0..n {
            let num = nums[right];
            current += num;
            if !seen.contains(&num) {
                result = result.max(current);
                seen.insert(num);
            } else {
                loop {
                    let removal = nums[left];
                    current -= removal;
                    left += 1;
                    if removal != num {
                        seen.remove(&removal);
                    } else {
                        break;
                    }
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
        let nums = vec![4,2,4,5,6];
        let result = Solution::maximum_unique_subarray(nums);
        assert_eq!(result, 17);
    }

    #[test]
    fn example_2() {
        let nums = vec![5,2,1,2,5,2,1,2,5];
        let result = Solution::maximum_unique_subarray(nums);
        assert_eq!(result, 8);
    }

}
