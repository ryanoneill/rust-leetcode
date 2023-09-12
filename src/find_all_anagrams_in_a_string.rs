use std::collections::HashMap;

/// Given two strings `s` and `p`, return an array of all the start indices of `p`'s anagrams in
/// `s`. You may return the answer in any order.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
struct Solution;

impl Solution {

    fn to_counts(p: &str) -> HashMap<char, usize> {
        let mut result = HashMap::new();

        for c in p.chars() {
            Self::add(&mut result, c);
        }

        result
    }

    fn add(counts: &mut HashMap<char, usize>, letter: char) {
        counts
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn remove(counts: &mut HashMap<char, usize>, letter: char) {
        if counts.contains_key(&letter) {
            let count = counts[&letter];
            if count == 1 {
                counts.remove(&letter);
            } else {
                counts
                    .entry(letter)
                    .and_modify(|count| *count -= 1);
            }
        }
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();

        let n = s.len();
        let m = p.len();

        if m <= n {
            let counts = Self::to_counts(&p);

            let mut current = Self::to_counts(&s[0..m]);
            if current == counts {
                result.push(0);
            }


            let letters: Vec<char> = s.chars().collect();
            for i in 1..n-m+1 {
                let old = letters[i-1];
                let new = letters[i+m-1];

                Self::add(&mut current, new);
                Self::remove(&mut current, old);

                if current == counts {
                    result.push(i as i32);
                }
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
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, vec![0,6]);
    }

    #[test]
    fn example_2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, vec![0,1,2]);
    }

    #[test]
    fn example_3() {
        let s = "aaaaaaaaaa".to_string();
        let p = "aaaaaaaaaaaaa".to_string();
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, Vec::new());
    }

}
