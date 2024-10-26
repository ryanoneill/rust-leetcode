use std::collections::HashMap;

/// Given a string `s`, partition `s` such that every substring of the partition is a palindrome.
///
/// Return the minimum cuts needed for a palindrome partitioning of `s`.
struct Solution;

impl Solution {

    fn is_palindrome(letters: &Vec<char>, start: usize, end: usize) -> bool {
        let mut left = start;
        let mut right = end;
        let mut result = true;

        while left < right {
            if letters[left] != letters[right] {
                result = false;
                break;
            }
            left += 1;
            right -= 1;
        }

        result
    }

    fn worker(
        results: &mut HashMap<usize, i32>,
        index: usize,
        letters: &Vec<char>
    ) -> i32 {
        let mut result = i32::MAX;
        let n = letters.len();
        if results.contains_key(&index) {
            result = results[&index]
        } else {
            if index == n {
                result = 0;
            } else if index < n {
                for i in index..n {
                    if Self::is_palindrome(letters, index, i) {
                        let attempt = Self::worker(results, i+1, letters);
                        if attempt != i32::MAX {
                            result = result.min(1 + attempt);
                        }
                    }
                }
            }
            results.insert(index, result);
        }
        result
    }

    pub fn min_cut(s: String) -> i32 {
        let mut results = HashMap::new();
        let letters = s.chars().collect();

        Self::worker(&mut results, 0, &letters) - 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("aab");
        let result = Solution::min_cut(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let s = str!("a");
        let result = Solution::min_cut(s);
        assert_eq!(result, 0);
    }
    
    #[test]
    fn example_3() {
        let s = str!("ab");
        let result = Solution::min_cut(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let s = str!("ababababababababababababcbabababababababababababa");
        let result = Solution::min_cut(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_5() {
        let s = str!("apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp");
        let result = Solution::min_cut(s);
        assert_eq!(result, 452);
    }

}
