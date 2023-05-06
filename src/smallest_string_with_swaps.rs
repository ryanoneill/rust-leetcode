use crate::union_find::UnionFind;

/// You are given a string `s`, and an array of pairs of indices in the string
/// `pairs` where `pairs[i] = [a, b]` indicates 2 indeices (0-indexed) of the
/// string.
///
/// You can swap the characters at any pair of indices in the given `pairs` any
/// number of times.
///
/// Return the lexicographically smallest string that `s` can be changed to
/// after using the swaps.
struct Solution;

impl Solution {

    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut uf = UnionFind::new(n);
        for pair in pairs {
            let x = pair[0];
            let y = pair[1];
            uf.union(x, y);
        }

        let components = uf.components();
        for mut component in components {
            component.sort();
            let mut piece: Vec<char> = component.iter().map(|i| chars[(*i) as usize]).collect();
            piece.sort();
            for i in 0..piece.len() {
                chars[component[i] as usize] = piece[i];
            }
        }

        chars.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0,3], vec![1,2]];
        let result = Solution::smallest_string_with_swaps(s, pairs);
        assert_eq!(result, "bacd");
    }

    #[test]
    fn example_2() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0,3], vec![1,2], vec![0,2]];
        let result = Solution::smallest_string_with_swaps(s, pairs);
        assert_eq!(result, "abcd");
    }

    #[test]
    fn example_3() {
        let s = "cba".to_string();
        let pairs = vec![vec![0,1], vec![1,2]];
        let result = Solution::smallest_string_with_swaps(s, pairs);
        assert_eq!(result, "abc");
    }

}
