use std::collections::HashMap;

/// You are given an array `nums` consisting of positive integers.
///
/// Return the total frequencies of elements in `nums` such that those elements all have the
/// maximum frequency.
///
/// The frequency of an element is the number of occurrences of that element in the array.
struct Solution;

impl Solution {

    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for num in nums {
            counts
                .entry(num)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut max = 0;
        let mut count = 0;

        for freq in counts.into_values() {
            if freq > max {
                max = freq;
                count = 1;
            } else if freq == max {
                count += 1;
            }
        }

        max * count
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,2,3,1,4];
        let result = Solution::max_frequency_elements(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,5];
        let result = Solution::max_frequency_elements(nums);
        assert_eq!(result, 5);
    }

}
