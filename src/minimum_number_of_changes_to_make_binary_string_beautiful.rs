/// You are given a 0-indexed binary string `s` having an even length.
///
/// A string is beautiful if it's possible to partition it into one or more substrings such that:
///
/// * Each substring has an even length.
/// * Each substring contains only `1`'s or only `0`'s.
///
/// You can change any character in `s` to `0` or `1`.
///
/// Return the minimum number of changes required to make the string `s` beautiful.
struct Solution;

impl Solution {

    pub fn min_changes(s: String) -> i32 {
        let mut result = 0;
        let numbers: Vec<char> = s.chars().collect();
        let n = numbers.len();
        let half = n / 2;

        for i in 0..half {
            let first = numbers[i * 2];
            let second = numbers[i * 2 + 1];
            if first != second {
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
        let s = str!("1001");
        let result = Solution::min_changes(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let s = str!("10");
        let result = Solution::min_changes(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let s = str!("0000");
        let result = Solution::min_changes(s);
        assert_eq!(result, 0);
    }


}
