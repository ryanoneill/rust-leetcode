use std::collections::HashMap;

struct TrieNode {
    value: char,
    items: HashMap<char, TrieNode>,
    ends_at: bool,
}

impl TrieNode {

    fn new(value: char) -> Self {
        TrieNode { value, items: HashMap::new(), ends_at: false }
    }

}

/// Design a data structure that supports adding new words and finding if a string matches any
/// previously added string.
///
/// Implement the `WordDictionary` class:
///
/// * `WordDictionary()` Initializes the object.
///
/// * `void addWord(word)` Adds `word` to the data structure, it can be matched later.
///
/// * `bool search(word)` Returns `true` if there is any string in the data structure that matches
///   `word` or `false` otherwise. `word` may contain dots `'.'` where dots can be matched with any
///   letter.
struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {

    fn new() -> Self {
        Self { root: TrieNode::new('*') }
    }

    fn add_word(&mut self, word: String) {
        let mut current: &mut TrieNode = &mut self.root;
        for letter in word.chars() {
            if current.items.contains_key(&letter) {
                current = current.items.get_mut(&letter).unwrap();
            } else {
                let node = TrieNode::new(letter);
                current.items.insert(letter, node);
                current = current.items.get_mut(&letter).unwrap();
            }
        }
        current.ends_at = true;
    }

    fn search(&self, word: String) -> bool {
        let letters: Vec<char> = word.chars().collect();
        Self::search_worker(&self.root, &letters, 0)
    }

    fn search_worker(node: &TrieNode, letters: &Vec<char>, i: usize) -> bool {
        let n = letters.len();
        let letter = letters[i];
        
        let mut result = false;
        if letter == '.' {
            for value in node.items.values() {
                if i == n-1 {
                    result = result || value.ends_at;
                } else {
                    result = result || Self::search_worker(value, letters, i+1);
                }
            }
        } else if node.items.contains_key(&letter) {
            let value = &node.items[&letter];
            if i == n-1 {
                result = value.ends_at;
            } else {
                result = Self::search_worker(value, letters, i+1);
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn example_1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());
        let result = word_dictionary.search("pad".to_string());
        assert!(!result);
        let result = word_dictionary.search("bad".to_string());
        assert!(result);
        let result = word_dictionary.search(".ad".to_string());
        assert!(result);
        let result = word_dictionary.search("b..".to_string());
        assert!(result);
    }

}
