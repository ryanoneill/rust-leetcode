use std::collections::HashMap;

struct DictionaryNode {
    value: char,
    items: HashMap<char, DictionaryNode>,
    ends_at: bool,
    word: String,
}

impl DictionaryNode {

    fn new(value: char) -> Self {
        Self {
            value,
            items: HashMap::new(),
            ends_at: false,
            word: String::from(""),
        }
    }
}

struct Dictionary {
    root: DictionaryNode,
}

impl Dictionary {

    fn new() -> Self {
        Self {
            root: DictionaryNode::new('*')
        }
    }

    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for letter in word.chars() {
            if !current.items.contains_key(&letter) {
                let node = DictionaryNode::new(letter);
                current.items.insert(letter, node);
            }
            current = current.items.get_mut(&letter).unwrap();
        }
        current.ends_at = true;
        current.word = word;
    }

    fn replace_word<'a>(&'a self, word: &'a str) -> &'a str {
        let mut result = word;
        let mut current = &self.root;
        for letter in word.chars() {
            if current.items.contains_key(&letter) {
                current = &current.items[&letter];
                if current.ends_at {
                    result = &current.word;
                    break;
                }
            } else {
                break;
            }
        }
        result
    }

}

/// In English, we have a concept called root, which can be followed by some other word to form
/// another longer word - let's call this word derivative. For example, when the root `"help"` is
/// followed by the word `"ful"`, we can form a derivative `"helpful"`.
///
/// Given a `dictionary` consisting of many roots and a `sentence` consisting of words separated by
/// spaces, replace all the derivatives in the sentence with the root forming it. If a derivative
/// can be replaced by more than one root, replace it with the root that has the shortest length.
///
/// Return the `sentence` after the replacement.
struct Solution;

impl Solution {

    fn to_dictionary(words: Vec<String>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            result.add_word(word);
        }
        result
    }

    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let dict = Self::to_dictionary(dictionary);
        let result: Vec<&str> = sentence
            .split(' ')
            .map(|word| dict.replace_word(word))
            .collect();
        result.join(" ")
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let dictionary = vec![str!("cat"), str!("bat"), str!("rat")];
        let sentence = str!("the cattle was rattled by the battery");
        let result = Solution::replace_words(dictionary, sentence);
        assert_eq!(result, "the cat was rat by the bat");
    }

    #[test]
    fn example_2() {
        let dictionary = vec![str!("a"), str!("b"), str!("c")];
        let sentence = str!("aadsfasf absbs bbab cadsfafs");
        let result = Solution::replace_words(dictionary, sentence);
        assert_eq!(result, "a a b c");
    }

}
