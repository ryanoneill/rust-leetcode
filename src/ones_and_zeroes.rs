use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    index: usize,
    rem_zeroes: usize,
    rem_ones: usize,
}

impl State {

    fn new(index: usize, rem_zeroes: usize, rem_ones: usize) -> Self {
        Self { index, rem_zeroes, rem_ones }
    }

    fn with(&self, item: &Item) -> Self {
        let new_rem_zeroes = self.rem_zeroes - item.zeroes;
        let new_rem_ones = self.rem_ones - item.ones;
        Self::new(self.index + 1, new_rem_zeroes, new_rem_ones)
    }

    fn without(&self) -> Self {
        Self::new(self.index + 1, self.rem_zeroes, self.rem_ones)
    }

    fn can_add(&self, item: &Item) -> bool {
        self.rem_zeroes >= item.zeroes && self.rem_ones >= item.ones
    }

}

#[derive(Debug)]
struct Item {
    zeroes: usize,
    ones: usize,
    orig: String
}

impl Item {

    fn new(s: String) -> Self {
        let mut ones = 0;
        let mut zeroes = 0;

        for c in s.chars() {
            match c {
                '0' => { 
                    zeroes += 1;
                }
                '1' => {
                    ones += 1;
                }
                _ => { }
            }
        }
        Self { ones, zeroes, orig: s }
    }

}

/// You are given an array of binary strings `strs` and two integers `m` and `n`.
///
/// Return the size of the largest subset of `strs` such that there are at most `m` `0`'s and `n`
/// `1`'s in the subset.
///
/// A set `x` is a subset of a set `y` if all elements of `x` are also elements of `y`.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, items: &Vec<Item>, state: State) -> i32 {
        if results.contains_key(&state) {
            results[&state]
        } else {
            let mut result = 0;

            let n = items.len();
            let current = &items[state.index];
            if state.index == n-1 {
                if state.can_add(current) {
                    result = 1;
                } else {
                    result = 0;
                }
            } else {
                if state.can_add(current) {
                    let with_state = state.with(current);
                    result = 1 + Self::worker(results, items, with_state);
                }
                result = result.max(Self::worker(results, items, state.without()));
            }

            results.insert(state, result);

            result
        }
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let rem_zeroes = m as usize;
        let rem_ones = n as usize;
        let items: Vec<Item> = strs.into_iter().map(Item::new).collect();
        let initial_state = State::new(0, rem_zeroes, rem_ones);
        let mut results = HashMap::new();
        Self::worker(&mut results, &items, initial_state)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let strs = vec!["10", "0001", "111001", "1", "0"];
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::find_max_form(strs, 5, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let strs = vec!["10", "0", "1"];
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::find_max_form(strs, 1, 2);
        assert_eq!(result, 2);
    }

}
