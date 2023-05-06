use std::collections::HashSet;

/// You're given strings `jewels` representing the types of stones that are
/// jewels, and `stones` representing the stones you have. Each character
/// is a type of stone you have. You want to know how many of the stones you
/// have are also jewels.
///
/// Letters are case sensitive, so `"a"` is considered a different type of
/// stone from `"A"`.
struct Solution;

impl Solution {

    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let js: HashSet<char> = HashSet::from_iter(jewels.chars());

        let mut result = 0;
        for stone in stones.chars() {
            if js.contains(&stone) {
                result += 1;
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
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        let result = Solution::num_jewels_in_stones(jewels, stones);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        let result = Solution::num_jewels_in_stones(jewels, stones);
        assert_eq!(result, 0);
    }

}
