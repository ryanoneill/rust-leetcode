/// You are given a string `s` and an integer array `indices` of the same length. The string `s`
/// will be shuffled such that the character at the `ith` position moves to `indices[i]` in the
/// shuffled string.
///
/// Return the shuffled string.
struct Solution;

impl Solution {

    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let n = indices.len();
        let letters: Vec<char> = s.chars().collect();

        let mut results = vec![' '; n];
        for i in 0..n {
            let letter = letters[i];
            let index = indices[i] as usize;
            results[index] = letter;
        }

        results.into_iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "codeleet".to_string();
        let indices = vec![4,5,6,7,0,2,1,3];
        let result = Solution::restore_string(s, indices);
        assert_eq!(result, "leetcode");
    }

    #[test]
    fn example_2() {
        let s = "abc".to_string();
        let indices = vec![0,1,2];
        let result = Solution::restore_string(s, indices);
        assert_eq!(result, "abc");
    }

}
