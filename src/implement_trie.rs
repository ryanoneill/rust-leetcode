use std::collections::HashMap;

/// A trie or prefix tree is a tree data structure used to efficiently store
/// and retrieve keys in a dataset of strings. There are various applications
/// of this data structure, such as autocomplete and spellchecker.
///
/// Implement the Trie class:
///
/// * `Trie()` Initializes the trie object.
///
/// * `void insert(String word)` Inserts the string `word` into the trie.
///
/// * `boolean search(String word)` Returns `true` if the string `word` is in
///   the trie (i.e., was inserted before) and `false` otherwise.
///
/// * `boolean startsWith(String prefix)` Returns `true` if there is a 
///   previously inserted string `word` that has the prefix `prefix`, and
///   `false` otherwise.
struct Trie {
    root: TrieNode,
}

impl Trie {

    fn new() -> Self {
        Trie { root: TrieNode::new('*') }
    }

    fn insert(&mut self, word: String) {
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
        let mut current = &self.root;
        let mut result = true;
        for letter in word.chars() {
            match current.items.get(&letter) {
                Some(value) => current = value,
                None => {
                    result = false;
                    break;
                }
            }
        }
        result && current.ends_at
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        let mut result = true;
        for letter in prefix.chars() {
            match current.items.get(&letter) {
                Some(value) => current = value,
                None => {
                    result = false;
                    break;
                }
            }
        }
        result
    }

}

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

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn example() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        let result1 = trie.search("apple".to_string());
        assert!(result1);
        let result2 = trie.search("app".to_string());
        assert!(!result2);
        let result3 = trie.starts_with("app".to_string());
        assert!(result3);
        trie.insert("app".to_string());
        let result4 = trie.search("app".to_string());
        assert!(result4);
    }

}
