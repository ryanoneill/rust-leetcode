use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    count: usize,
    word: String,
}

impl State {
    fn new(word: String) -> Self {
        State { count: 1, word }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // Word ordering is reversed so that when the States have the
    // same count that the words are ordered in lexicographical order.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count < other.count {
            Ordering::Less
        } else if self.count > other.count {
            Ordering::Greater
        } else if self.word < other.word {
            Ordering::Greater
        } else if self.word > other.word {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

/// Given an array of strings `words` and an integer `k`, return the `k` most
/// frequent strings.
///
/// Return the answer sorted by the frequency from highest to lowest. Sort the
/// words with the same frequency by their lexicographical order.
struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counts: HashMap<String, State> = HashMap::new();

        for word in words {
            counts
                .entry(word.clone())
                .and_modify(|s| {
                    s.increment();
                })
                .or_insert(State::new(word));
        }

        let mut max_heap = BinaryHeap::from_iter(counts.into_values());
        let mut result = Vec::new();
        for _ in 0..k as usize {
            let state = max_heap.pop().unwrap();
            result.push(state.word);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let orig = vec!["i", "love", "leetcode", "i", "love", "coding"];
        let words: Vec<String> = orig.iter().map(|s| s.to_string()).collect();
        let k = 2;
        let result = Solution::top_k_frequent(words, k);
        assert_eq!(result, vec!["i", "love"]);
    }

    #[test]
    fn example_2() {
        let orig = vec![
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ];
        let words: Vec<String> = orig.iter().map(|s| s.to_string()).collect();
        let k = 4;
        let result = Solution::top_k_frequent(words, k);
        assert_eq!(result, vec!["the", "is", "sunny", "day"]);
    }
}
