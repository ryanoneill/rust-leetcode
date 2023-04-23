/// Design an algorithm that collects daily price quotes for some stock and
/// returns the span of that stock's price for the current day.
///
/// The span of the stock's price in one day is the maximum number of
/// consecutive days (starting from that day and going backward) for which the
/// stock price was less than or equal to the price of that day.
///
/// * For example, if the prices of the stock in the last four days is
///   `[7, 2, 1, 2]` and the price of the stock today is `2`, then the span of
///   today is `4` because starting from today, the price of the stock was less
///   than or equal `2` for `4` consecutive days.
///
/// * Also, if the prices of the stock in the last four days is `[7, 34, 1, 2]`
///   and the price of the stock today is `8`, then the span of today is `3`
///   because starting from today, the price of the stock was less than or equal
///   `8` for `3` consecutive days.
///
/// Implement the `StockSpanner` class:
/// * `StockSpanner()` Initializes the object of the class.
/// * `int next(int price)` Returns the span of the stock's price given that
///   today's price is `price`.
///
struct StockSpanner {
    prices: Vec<(usize, i32)>,
    day: usize
}

impl StockSpanner {

    fn new() -> Self {
        StockSpanner { prices: vec![], day: 0 }
    }

    fn has_non_greater_price(&mut self, price: i32) -> bool {
        if self.prices.is_empty() { false }
        else {
            let last_index = self.prices.len() - 1;
            let last_value = self.prices[last_index];
            last_value.1 <= price
        }
    }

    fn remove_non_greater_prices(&mut self, price: i32) {
        while self.has_non_greater_price(price) {
            self.prices.pop();
        }
    }

    fn get_last_day_with_greater_price(&self) -> usize {
        let last_index = self.prices.len() - 1;
        let last_value = self.prices[last_index];
        last_value.0
    }

    fn next(&mut self, price: i32) -> i32 {
        self.day += 1;
        self.remove_non_greater_prices(price);
        if self.prices.is_empty() {
            self.prices.push((self.day, price));
            self.day as i32
        } else {
            let last_day = self.get_last_day_with_greater_price();
            self.prices.push((self.day, price));
            (self.day - last_day) as i32
        }
    }

}

#[cfg(test)]
mod tests {
    use super::StockSpanner;

    #[test]
    fn example_1() {
        let mut stock_spanner = StockSpanner::new();
        let result1 = stock_spanner.next(100);
        assert_eq!(result1, 1);
        let result2 = stock_spanner.next(80);
        assert_eq!(result2, 1);
        let result3 = stock_spanner.next(60);
        assert_eq!(result3, 1);
        let result4 = stock_spanner.next(70);
        assert_eq!(result4, 2);
        let result5 = stock_spanner.next(60);
        assert_eq!(result5, 1);
        let result6 = stock_spanner.next(75);
        assert_eq!(result6, 4);
        let result7 = stock_spanner.next(85);
        assert_eq!(result7, 6);
    }

    #[test]
    fn decreasing() {
        let mut stock_spanner = StockSpanner::new();
        let result1 = stock_spanner.next(100);
        assert_eq!(result1, 1);
        let result2 = stock_spanner.next(90);
        assert_eq!(result2, 1);
        let result3 = stock_spanner.next(80);
        assert_eq!(result3, 1);
        let result4 = stock_spanner.next(70);
        assert_eq!(result4, 1);
    }

    #[test]
    fn increasing() {
        let mut stock_spanner = StockSpanner::new();
        let result1 = stock_spanner.next(50);
        assert_eq!(result1, 1);
        let result2 = stock_spanner.next(60);
        assert_eq!(result2, 2);
        let result3 = stock_spanner.next(70);
        assert_eq!(result3, 3);
        let result4 = stock_spanner.next(80);
        assert_eq!(result4, 4);
        let result5 = stock_spanner.next(90);
        assert_eq!(result5, 5);
    }

    #[test]
    fn same() {
        let mut stock_spanner = StockSpanner::new();
        let result1 = stock_spanner.next(50);
        assert_eq!(result1, 1);
        let result2 = stock_spanner.next(50);
        assert_eq!(result2, 2);
        let result3 = stock_spanner.next(50);
        assert_eq!(result3, 3);
        let result4 = stock_spanner.next(40);
        assert_eq!(result4, 1);
        let result5 = stock_spanner.next(60);
        assert_eq!(result5, 5);
    }

}
