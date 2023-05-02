/// You are given an array `prices` where `prices[i]` is the price of a given
/// stock on the `ith` day.
///
/// You want to maximize your profit by choosing a single day to buy one stock
/// and choosing a different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from this transaction. If you
/// cannot achieve any profit, return `0`.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn max_profit(_prices: Vec<i32>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let prices = vec![7,1,5,3,6,4];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 5);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let prices = vec![7,6,4,3,1];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 0);
    }

}
