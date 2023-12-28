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

    pub fn next_day(&self) -> Self {
        Self::new(self.index + 1, self.holding)
    }

    pub fn sell(&self) -> Self {
        Self::new(self.index + 1, false)
    }

    pub fn buy(&self) -> Self {
        Self::new(self.index + 1, true)
    }

}

/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `ith`
/// day, and an integer `fee` representing a transaction fee.
///
/// Find the maximum profit you can achieve. You may complete as many transactions as you like, but
/// you need to pay the transaction fee for each transaction.
///
/// Note:
///
/// * You may not engage in multiple transactions simultaneously (i.e., you must sell the stock
///   before you buy again).
///
/// * The transaction fee is only charged once for each stock purchase and sale.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, prices: &Vec<i32>, fee: i32, state: State) -> i32 {
        let n = prices.len();

        if state.index == n {
            0
        } else if results.contains_key(&state) {
            results[&state]
        } else {
            let mut result = 0;
            if state.index == n-1 {
                if state.holding {
                    result -= fee;
                    result += prices[n-1];
                }
            } else {
                // If we hold stock, we can sell today
                // If we hold stock, we can go to the next day
                // If we hold no stock, we can buy today
                // If we hold no stock, we can go to the next day
                if state.holding {
                    let sell_state = state.sell();
                    let mut sell_result = 0;
                    sell_result -= fee;
                    sell_result += prices[state.index];
                    sell_result += Self::worker(results, prices, fee, sell_state);

                    let skip_state = state.next_day();
                    let skip_result = Self::worker(results, prices, fee, skip_state);

                    result = sell_result.max(skip_result);
                } else {
                    let buy_state = state.buy();
                    let mut buy_result = 0;
                    buy_result -= prices[state.index];
                    buy_result += Self::worker(results, prices, fee, buy_state);

                    let skip_state = state.next_day();
                    let skip_result = Self::worker(results, prices, fee, skip_state);

                    result = buy_result.max(skip_result);
                }
            }

            results.insert(state, result);
            result
        }
    }

    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let initial = State::initial();
        let mut results = HashMap::new();
        let result = Self::worker(&mut results, &prices, fee, initial);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let prices = vec![1,3,2,8,4,9];
        let fee = 2;
        let result = Solution::max_profit(prices, fee);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let prices = vec![1,3,7,5,10,3];
        let fee = 3;
        let result = Solution::max_profit(prices, fee);
        assert_eq!(result, 6);
    }

}
