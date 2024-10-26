use std::collections::HashSet;

struct Solution;

impl Solution {

    fn worker(seen: &mut HashSet<String>, letters: &Vec<char>, index: usize) -> i32 {
        let n = letters.len();

        let mut result = i32::MIN;
        if index == n {
            result = 0;
        } else {
            for i in index..n {
                let word: String = letters[index..=i].iter().collect();
                if !seen.contains(&word) {
                    seen.insert(word.clone());
                    let attempt = Self::worker(seen, letters, i+1);
                    if attempt != i32::MIN {
                        result = result.max(1 + attempt);
                    }
                    seen.remove(&word);
                }
            }
        }

        result
    }

    pub fn max_unique_split(s: String) -> i32 {
        let mut seen = HashSet::new();
        let letters = s.chars().collect();

        Self::worker(&mut seen, &letters, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("ababccc");
        let result = Solution::max_unique_split(s);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let s = str!("aba");
        let result = Solution::max_unique_split(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let s = str!("aa");
        let result = Solution::max_unique_split(s);
        assert_eq!(result, 1);
    }

}
