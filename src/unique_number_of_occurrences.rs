use std::collections::HashMap;
use std::collections::HashSet;

/// Given an array of integers `arr`, return `true` if the
/// number of occurrences of each value in the array is
/// unique or `false` otherwise.
struct Solution;

impl Solution {

    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts = HashMap::new();
        for num in arr {
            counts
                .entry(num)
                .and_modify(|count| { *count += 1; })
                .or_insert(1);
        }
        let n = counts.len();
        let unique: HashSet<i32> = HashSet::from_iter(counts.into_values());

        n == unique.len()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let arr = vec![1,2,2,1,1,3];
        let result = Solution::unique_occurrences(arr);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let arr = vec![1,2];
        let result = Solution::unique_occurrences(arr);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let arr = vec![-3,0,1,-3,1,1,1,-3,10,0];
        let result = Solution::unique_occurrences(arr);
        assert!(result);
    }

}
