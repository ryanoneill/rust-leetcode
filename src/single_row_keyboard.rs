use std::collections::HashMap;

/// There is a special keyboard with all keys in a single row.
///
/// Given a string `keyboard` of length `26` indicating the layout of the keyboard (indexed from
/// `0` to `25`). Initially, your finger is at index `0`. To type a character, you have to move
/// your finger to the index of the desired character. The time taken to move your finger from
/// index `i` to index `j` is `|i - j|`.
///
/// You want to type a string `word`. Write a function to calculate how much time it takes to type
/// it with one finger.
struct Solution;

impl Solution {

    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut index = 0;
        let mut indices: HashMap<char, usize> = HashMap::new();
        for c in keyboard.chars() {
            indices.insert(c, index);
            index += 1;
        }

        let mut result = 0;
        let mut i = 0;
        for c in word.chars() {
            let j = indices[&c];
            result += (i.max(j) - i.min(j)) as i32;
            i = j;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let keyboard = str!("abcdefghijklmnopqrstuvwxyz");
        let word = str!("cba");
        let result = Solution::calculate_time(keyboard, word);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let keyboard = str!("pqrstuvwxyzabcdefghijklmno");
        let word = str!("leetcode");
        let result = Solution::calculate_time(keyboard, word);
        assert_eq!(result, 73);
    }

}
