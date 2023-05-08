/// You are given an integer array `prices` where `prices[i]` is the price of a
/// given stock on the `ith` day.
///
/// On each day, you may decide to buy and/or sell the stock. You can only hold
/// at most one share of stock at any time. However, you can buy it then
/// immediately sell it on the same day.
///
/// Find and return the maximum profit you can achieve.
struct Solution;

impl Solution {

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut result = 0;
        let mut previous = prices[0];

        for i in 1..n {
            let today = prices[i];
            let diff = today - previous;
            if diff > 0 {
                result += diff;
            }
            previous = today;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn exmaple_1() {
        let prices = vec![7,1,5,3,6,4];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let prices = vec![1,2,3,4,5];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let prices = vec![7,6,4,3,1];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 0);
    }

}
