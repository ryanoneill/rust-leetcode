use std::collections::HashSet;

/// Given an integer array `nums` and an integer `k`, return `true` if there
/// are two distinct indices `i` and `j` in the array such that
/// `nums[i] == nums[j]` and `abs(i - j) <= k`.
struct Solution;

impl Solution {

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;
        let mut seen = HashSet::new();
        let mut result = false;

        if k > 0 {
            let m = k.min(n);
            for i in 0..m {
                let num = nums[i];
                if seen.contains(&num) {
                    result = true;
                    break;
                } else {
                    seen.insert(num);
                }
            }

            if !result {
                for i in m..n {
                    let num = nums[i];
                    if seen.contains(&num) {
                        result = true;
                        break;
                    } else {
                        let old = nums[i - m];
                        seen.remove(&old);
                        seen.insert(num);
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
        let nums = vec![1,2,3,1];
        let k = 3;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,0,1,1];
        let k = 1;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2,3,1,2,3];
        let k = 2;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert!(!result);
    }

    #[test]
    fn example_4() {
        let nums = vec![1,2,1];
        let k = 0;
        let result = Solution::contains_nearby_duplicate(nums, k);
        assert!(!result);
    }

}
