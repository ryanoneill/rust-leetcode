use std::collections::HashSet;

/// Write an algorithm to determine if a number `n` is happy.
///
/// A happy number is a number defined by the following process:
///
/// * Starting with any positive integer, replace the number by the sum of the
///   squares of its digits.
///
/// * Repeat the process until the number equals 1 (where it will stay), or it
///   loops endlessly in a cycle which does not include 1.
///
/// * Those numbers for which this process ends in 1 are happy.
///
/// Return `true` if `n` is a happy number, and `false` if not.
struct Solution;

impl Solution {

    pub fn is_happy(n: i32) -> bool {
        let mut num = n;
        let mut seen = HashSet::new();

        while !seen.contains(&num) && num != 1 {
            seen.insert(num);
            num = num.to_string()
                .split("")
                .map(|d| d.parse::<i32>().unwrap_or_default())
                .map(|d| d * d)
                .sum();
        }

        num == 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 19;
        let result = Solution::is_happy(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 2;
        let result = Solution::is_happy(n);
        assert!(!result);
    }

}
