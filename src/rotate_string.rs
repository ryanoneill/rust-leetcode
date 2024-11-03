/// Given two strings `s` and `goal`, return `true` if an only if `s` can become `goal` after some
/// number of shifts on `s`.
///
/// A shift on `s` consists of moving the leftmost character of `s` to the rightmost position.
///
/// * For example, if `s = "abcde"`, then it will be "bcdea" after one shift.
struct Solution;

impl Solution {

    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut result = s.len() == goal.len();
        if result {
            let doubled = s.clone() + &s;
            result = doubled.find(&goal).is_some()
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("abcde");
        let goal = str!("cdeab");
        let result = Solution::rotate_string(s, goal);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let s = str!("abcde");
        let goal = str!("abced");
        let result = Solution::rotate_string(s, goal);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let s = str!("aa");
        let goal = str!("a");
        let result = Solution::rotate_string(s, goal);
        assert!(!result);
    }

}
