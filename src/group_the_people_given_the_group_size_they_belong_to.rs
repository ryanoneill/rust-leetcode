use std::collections::HashMap;

/// There are `n` people that are split into some unknown number of groups. Each person is labeled
/// with a unique ID from `0` to `n-1`.
///
/// You are given an integer array `groupSizes`, where `groupSizes[i]` is the size of the group
/// that person `i` is in in.
///
/// Return a list of groups such that each person `i` is in a group of size `groupSizes[i]`.
///
/// Each person should appear in exactly one group, and every person must be in a group. If there
/// are multiple answers, return any of them. It is guaranteed that there will be at least one
/// valid solution for the given input.
struct Solution;

impl Solution {

    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let n = group_sizes.len();
        let mut counts = HashMap::new();

        for i in 0..n {
            let size = group_sizes[i];
            counts
                .entry(size)
                .or_insert(Vec::new())
                .push(i as i32);

            let m = counts[&size].len();
            if m == size as usize {
                let items = counts.remove(&size).unwrap();
                results.push(items);
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let group_sizes = vec![3,3,3,3,3,1,3];
        let mut result = Solution::group_the_people(group_sizes);
        result.sort_unstable();
        assert_eq!(result, vec![vec![0,1,2], vec![3,4,6], vec![5]]);
    }

    #[test]
    fn example_2() {
        let group_sizes = vec![2,1,3,3,3,2];
        let mut result = Solution::group_the_people(group_sizes);
        result.sort_unstable();
        assert_eq!(result, vec![vec![0,5], vec![1], vec![2,3,4]]);
    }

}
