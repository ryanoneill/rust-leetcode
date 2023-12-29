use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    index: usize,
    holding: bool,
}

impl State {

    pub fn new(index: usize, holding: bool) -> Self {
        Self { index, holding }
    }

    pub fn initial() -> Self {
        Self::new(0, false)
    }

    pub fn skip(&self) -> Self {
        Self::new(self.index + 1, self.holding)
    }

    pub fn sell(&self) -> Self {
        // Add two to skip over the cooldown day
        Self::new(self.index + 2, false)
    }

    pub fn buy(&self) -> Self {
        Self::new(self.index + 1, true)
    }

}

/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `ith`
/// day.
///
/// Find the maximum profit you can achieve. You may complete as many transactions as you like
/// (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
///
/// * After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
///
/// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock
/// before you buy again).
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, prices: &Vec<i32>, state: State) -> i32 {
        let n = prices.len();

        if state.index >= n {
            0
        } else if results.contains_key(&state) {
            results[&state]
        } else {
            let mut result = 0;
            if state.index == n-1 {
                if state.holding {
                    result += prices[n-1];
                }
            } else {
                if state.holding {
                    let sell_state = state.sell();
                    let mut sell_result = 0;
                    sell_result += prices[state.index];
                    sell_result += Self::worker(results, prices, sell_state);

                    let skip_state = state.skip();
                    let skip_result = Self::worker(results, prices, skip_state);

                    result = sell_result.max(skip_result);
                } else {
                    let buy_state = state.buy();
                    let mut buy_result = 0;
                    buy_result -= prices[state.index];
                    buy_result += Self::worker(results, prices, buy_state);

                    let skip_state = state.skip();
                    let skip_result = Self::worker(results, prices, skip_state);

                    result = buy_result.max(skip_result);
                }
            }

            results.insert(state, result);
            result
        }

    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let initial = State::initial();
        let mut results = HashMap::new();
        let result = Self::worker(&mut results, &prices, initial);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let prices = vec![1,2,3,0,2];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let prices = vec![1];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 0);
    }

}
