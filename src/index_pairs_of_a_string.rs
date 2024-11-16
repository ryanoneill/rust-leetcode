use std::collections::HashMap;

struct DictionaryNode {
    value: char,
    items: HashMap<char, DictionaryNode>,
    ends_at: bool,
}

impl DictionaryNode {

    pub fn new(value: char) -> Self {
        Self {
            value,
            items: HashMap::new(),
            ends_at: false,
        }
    }

}

struct Dictionary {
    root: DictionaryNode
}

impl Dictionary {

    pub fn new() -> Self {
        Self {
            root: DictionaryNode::new('*')
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for letter in word.chars() {
            if !current.items.contains_key(&letter) {
                let node = DictionaryNode::new(letter);
                current.items.insert(letter, node);
            }
            current = current.items.get_mut(&letter).unwrap();
        }
        current.ends_at = true;
    }

    pub fn search(
        &mut self,
        results: &mut Vec<Vec<i32>>,
        letters: &Vec<char>,
        start: usize
    ) {
        let n = letters.len();
        let mut current = &self.root;

        for i in start..n {
            let letter = letters[i];
            if current.items.contains_key(&letter) {
                current = &current.items[&letter];
            } else {
                break;
            }
            if current.ends_at {
                results.push(vec![start as i32, i as i32]);
            }
        }
    }

}

/// Given a string `text` and an array of strings `words`, return an array of all index pairs `[i,
/// j]` so that the substring `text[i...j]` is in `words`.
///
/// Return the pairs `[i,j]` in sorted order (i.e., sort them by their first coordinate, and in
/// case of ties sort them by their second coordinate).
struct Solution;

impl Solution {

    fn to_dictionary(words: Vec<String>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            result.add_word(word);
        }
        result
    }

    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();

        let mut dict = Self::to_dictionary(words);
        let letters: Vec<char> = text.chars().collect();
        let n = letters.len();

        for i in 0..n {
            dict.search(&mut results, &letters, i);
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let text = str!("thestoryofleetcodeandme");
        let words = vec![str!("story"), str!("fleet"), str!("leetcode")];
        let result = Solution::index_pairs(text, words);
        assert_eq!(result, vec![vec![3,7], vec![9,13], vec![10,17]]);
    }

    #[test]
    fn example_2() {
        let text = str!("ababa");
        let words = vec![str!("aba"), str!("ab")];
        let result = Solution::index_pairs(text, words);
        assert_eq!(result, vec![vec![0,1], vec![0,2], vec![2,3], vec![2,4]]);
    }

}
