use std::collections::HashMap;

/// Given a string `text`, you want to use the characters of `text` to form as many instances of
/// the word "balloon" as possible.
///
/// You can use each character in `text` at most once. Return the maximum number of instances that
/// can be formed.
struct Solution;

impl Solution {

    fn get_counts(text: String) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        result.insert('b', 0);
        result.insert('a', 0);
        result.insert('l', 0);
        result.insert('o', 0);
        result.insert('n', 0);

        for letter in text.chars() {
            match letter {
                'b' | 'a' | 'l' | 'o' | 'n' => {
                    if let Some(count) = result.get_mut(&letter) {
                        *count += 1;
                    }
                }
                _ => { }
            }
        }

        result
    }

    pub fn max_number_of_balloons(text: String) -> i32 {
        let counts = Self::get_counts(text);
        counts
            .into_iter()
            .map(|(k, v)| {
                match k {
                    'l' | 'o' => v / 2,
                    _ => v
                }
            })
            .min()
            .unwrap_or(0) as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let text = "nlaebolko".to_string();
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let text = "loonbalxballpoon".to_string();
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let text = "leetcode".to_string();
        let result = Solution::max_number_of_balloons(text);
        assert_eq!(result, 0);
    }

}
