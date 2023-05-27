use std::collections::HashMap;

/// You are given an integer array `nums`. You want to maximize the number of
/// points you get by performing the following operation any number of times:
///
/// * Pick any `nums[i]` and delete it to earn `nums[i]` points. Afterwards,
///   you must delete every element equal to `nums[i] - 1` and every element
///   equal to `nums[i] + 1`.
///
/// Return the maximum number of points you can earn by applying the above
/// operation some number of times.
struct Solution;

impl Solution {

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut max = 0;
        for num in nums {
            let num = num as usize;
            counts
                .entry(num)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            max = max.max(num);
        }

        let mut points = vec![0; max + 1];
        if counts.contains_key(&1) {
            points[1] = counts[&1];
        }
        for i in 2..=max {
            let without = points[i-1];
            let with = points[i-2] + counts.get(&i).map(|c| c * i as i32).unwrap_or_default();
            points[i] = with.max(without);
        }

        points[max]
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,4,2];
        let result = Solution::delete_and_earn(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,2,3,3,3,4];
        let result = Solution::delete_and_earn(nums);
        assert_eq!(result, 9);
    }

}
