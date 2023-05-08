struct Solution;

impl Solution {

    pub fn merge_alternately(word1: String, word2: String) -> String {
        let n = word1.len() + word2.len();
        let mut w1 = word1.chars();
        let mut w2 = word2.chars();

        let mut result = Vec::new();

        while result.len() < n {
            match (w1.next(), w2.next()) {
                (Some(a), Some(b)) => {
                    result.push(a);
                    result.push(b);
                }
                (Some(a), None) => {
                    result.push(a);
                }
                (None, Some(b)) => {
                    result.push(b);
                }
                _ => { break; }
            }
        }

        result.iter().collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let result = Solution::merge_alternately(word1, word2);
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn example_2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let result = Solution::merge_alternately(word1, word2);
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn example_3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let result = Solution::merge_alternately(word1, word2);
        assert_eq!(result, "apbqcd");
    }

}
