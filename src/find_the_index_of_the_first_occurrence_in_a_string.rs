/// Given two strings `needle` and `haystack`, return the index of the first
/// occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of
/// `haystack`.
struct Solution;

impl Solution {

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut result = -1;

        let h: Vec<char> = haystack.chars().collect();
        let n: Vec<char> = needle.chars().collect();

        let hlen = h.len();
        let nlen = n.len();

        for i in 0..hlen {
            let mut index = 0;
            while (index < nlen) && (i + index < hlen) && h[i+index] == n[index] {
                index += 1;
            }
            if index == nlen {
                result = i as i32;
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
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, -1);
    }

    #[test]
    fn middle() {
        let haystack = "goodbye".to_string();
        let needle = "od".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, 2);
    }

    #[test]
    fn end() {
        let haystack = "cartoon".to_string();
        let needle = "toon".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, 3);
    }

    #[test]
    fn off_end() {
        let haystack = "national".to_string();
        let needle = "ale".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, -1);
    }

}
