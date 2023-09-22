use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    count: usize,
    letter: char,
}

impl Item {

    pub fn new(letter: char) -> Self {
        Self { count: 1, letter }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

}

/// Given a character array `tasks`, representing the tasks a CPU needs to do, where each letter
/// represents a different task. Tasks could be done in any order. Each task is done in one unit of
/// time, the CPU could complete either one task or just be idle.
///
/// However, there is a non-negative integer `n` that represents the cooldown period between the
/// two same tasks (the same letter in the array), that is that there must be at least `n` units of
/// time between any two same tasks.
///
/// Return the least number of units of times that the CPU will take to finish all the given tasks.
struct Solution;

impl Solution {

    fn to_counts(tasks: Vec<char>) -> HashMap<char, Item> {
        let mut results = HashMap::new();

        for task in tasks {
            results
                .entry(task)
                .and_modify(|item: &mut Item| item.increment())
                .or_insert(Item::new(task));
        }

        results
    }

    fn to_heap(counts: HashMap<char, Item>) -> BinaryHeap<Item> {
        let mut results = BinaryHeap::new();

        for value in counts.values() {
            results.push(*value);
        }

        results
    }

    // The actual solution is much simpler than this in that the slots can
    // be prefilled by starting with the highest counts and scheduling them
    // for the slot in the future when they'll next be able to be run.
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut result = 0;
        let n = n as usize;

        let counts = Self::to_counts(tasks);
        let mut heap = Self::to_heap(counts);

        let mut queue: VecDeque<char> = VecDeque::new();
        let mut current: HashSet<char> = HashSet::new();

        while !heap.is_empty() {
            result += 1;

            if queue.len() == n+1 {
                let first = queue.pop_front().unwrap();
                if first != ' ' {
                    current.remove(&first);
                }
            }
            let mut options: Vec<Item> = Vec::new();
            let mut choice = ' ';
            while !heap.is_empty() {
                let mut option = heap.pop().unwrap();
                if current.contains(&option.letter) {
                    options.push(option);
                } else {
                    option.decrement();
                    choice = option.letter;
                    if !option.is_empty() {
                        options.push(option);
                    }
                    break;
                }
            }
            queue.push_back(choice);
            current.insert(choice);
            for option in options {
                heap.push(option);
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        // [A, B, _, A, B, _, A, B]
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;
        let result = Solution::least_interval(tasks, n);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        // [A, A, A, B, B, B]
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 0;
        let result = Solution::least_interval(tasks, n);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        // [A, B, C, A, D, E, A, F, G, A, _, _, A, _, _, A]
        let tasks = vec!['A','A','A','A','A','A','B','C','D','E','F','G'];
        let n = 2;
        let result = Solution::least_interval(tasks, n);
        assert_eq!(result, 16);
    }

}
