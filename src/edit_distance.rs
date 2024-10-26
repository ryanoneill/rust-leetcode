use std::collections::HashMap;

struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<(usize, usize), i32>, letters1: &Vec<char>, index1: usize, letters2: &Vec<char>, index2: usize) -> i32 {
        let result;

        let m = letters1.len();
        let n = letters2.len();

        let key = (index1, index2);

        if results.contains_key(&key) {
            result = results[&key];
        } else {
            if index1 == m {
                result = (n - index2) as i32;
            } else if index2 == n {
                result = (m - index1) as i32;
            } else if letters1[index1] == letters2[index2] {
                result = Self::worker(results, letters1, index1+1, letters2, index2+1);
            } else {
                // Insert a character in word 1
                let insert = 1 + Self::worker(results, letters1, index1, letters2, index2+1);

                // Delete a character in word 1
                let delete = 1 + Self::worker(results, letters1, index1+1, letters2, index2);

                // Replace a character in word 1
                let replace = 1 + Self::worker(results, letters1, index1+1, letters2, index2+1);

                result = insert.min(delete).min(replace);
            }
            results.insert(key, result);
        }

        result
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut results = HashMap::new();

        let letters1 = word1.chars().collect();
        let letters2 = word2.chars().collect();

        Self::worker(&mut results, &letters1, 0, &letters2, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let word1 = str!("horse");
        let word2 = str!("ros");
        let result = Solution::min_distance(word1, word2);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let word1 = str!("intention");
        let word2 = str!("execution");
        let result = Solution::min_distance(word1, word2);
        assert_eq!(result, 5);
    }

}
