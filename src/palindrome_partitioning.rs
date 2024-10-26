/// Given a string `s`, partition `s` such that every substring of the partition is a palindrome.
/// Return all possible palindrome partitioning of `s`.
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
        results: &mut Vec<Vec<String>>,
        part: &mut Vec<String>,
        index: usize,
        letters: &Vec<char>
    ) {
        let n = letters.len();
        if index == n {
            results.push(part.clone());
        } else {
            for i in index..n {
                if Self::is_palindrome(letters, index, i) {
                    part.push(letters[index..=i].iter().collect());
                    Self::worker(results, part, i+1, letters);
                    part.pop();
                }
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let mut part = Vec::new();
        let index = 0;
        let letters = s.chars().collect();
        Self::worker(&mut results, &mut part, index, &letters);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = str!("aab");
        let results = Solution::partition(s);
        assert_eq!(results, vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
    }

    #[test]
    fn example_2() {
        let s = str!("a");
        let results = Solution::partition(s);
        assert_eq!(results, vec![vec!["a"]]);
    }

}
