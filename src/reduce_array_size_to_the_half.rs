use std::collections::HashMap;

/// Return the minimum size of the set so that at least half of the integers of
/// the array are removed.
struct Solution; 

impl Solution {

    fn to_counts(arr: Vec<i32>) -> HashMap<i32, usize> {
        let mut result = HashMap::new();
        let n = arr.len();
        for i in 0..n {
            let value = arr[i];
            result
                .entry(value)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        result
    }

    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let half = n / 2;
        let target = n - half; // accounts for odd n's

        let counts = Self::to_counts(arr);

        let mut counts: Vec<usize> = counts.into_values().collect();
        // Highest first
        counts.sort_by(|a, b| b.cmp(a)); 

        let mut removed = 0;
        let mut result = 0;

        for i in 0..counts.len() {
            removed += counts[i];
            result += 1;
            if removed >= target {
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
        let arr = vec![3,3,3,3,5,5,5,2,2,7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let arr = vec![7,7,7,7,7,7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 1);
    }

}
