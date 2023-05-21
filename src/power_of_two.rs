use std::collections::HashSet;

/// Given an integer `n`, return `true` if it is a power of two. Otherwise,
/// return `false`.
///
/// An integer `n` is a power of two, if there exists an integer `x` such that
/// `n == 2^x`.
struct Solution;

impl Solution {

    // Structured this way in order to try using `POWERS` as a static
    // `HashSet<i32>`. Unfortunately, making this method a static
    // function is prohibited due to not allowing for loops in constant
    // functions (#87575) and also not allowing construction of mutable
    // `HashSet`s. The compiler helpfully recommends using the `once_cell`
    // crate, however that is unavailable for use with LeetCode.
    //
    // So, this solution non-optimally crates the HashSet each time
    // the `is_power_of_two` function is called. Very wasteful.
    // TODO: Check each number individually.
    fn build_powers() -> HashSet<i32> {
        let mut result = HashSet::new();
        let mut value = 1;
        result.insert(value);
        for _ in 1..31 {
            value = value * 2;
            result.insert(value);
        }
        result
    }

    pub fn is_power_of_two(n: i32) -> bool {
        Self::build_powers().contains(&n)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 1;
        let result = Solution::is_power_of_two(n);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 16;
        let result = Solution::is_power_of_two(n);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let result = Solution::is_power_of_two(n);
        assert!(!result);
    }

}
