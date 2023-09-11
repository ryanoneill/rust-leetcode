use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    index: usize,
    remaining: usize,
    holding: bool,
}

impl State {

    pub fn new(index: usize, remaining: usize, holding: bool) -> State {
        Self { index, remaining, holding }
    }

    pub fn next_day(&self) -> State {
        Self::new(self.index + 1, self.remaining, self.holding)
    }

    pub fn sell(&self) -> State {
        Self::new(self.index + 1, self.remaining - 1, false)
    }

    pub fn buy(&self) -> State {
        Self::new(self.index + 1, self.remaining, true)
    }

}

/// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the
/// `ith` day, and an integer `k`.
///
/// Find the maximum profit you can achieve. You may complete at most `k` transactions: i.e., you
/// may buy at most `k` times and sell at most `k` times.
///
/// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock
/// before you buy again).
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, prices: &Vec<i32>, state: State) -> i32 {
        let mut result = 0;
        if state.remaining == 0 {
            result
        } else if state.index == prices.len() {
            result
        } else if results.contains_key(&state) {
            results[&state]
        } else {

            let do_nothing = Self::worker(results, prices, state.next_day());
            let do_something;

            if state.holding {
                do_something = prices[state.index] + Self::worker(results, prices, state.sell());
            } else {
                do_something = -prices[state.index] + Self::worker(results, prices, state.buy());
            }

            result = do_nothing.max(do_something);
            results.insert(state, result);
            result
        }
    }


    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut results = HashMap::new();
        let initial_state = State::new(0, k, false);
        Self::worker(&mut results, &prices, initial_state)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let k = 2;
        let prices = vec![2,4,1];
        let result = Solution::max_profit(k, prices);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let k = 2;
        let prices = vec![3,2,6,5,0,3];
        let result = Solution::max_profit(k, prices);
        assert_eq!(result, 7);
    }

}
