#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    index: usize,
    price: i32,
}

impl Item {
    
    pub fn new(index: usize, price: i32) -> Self {
        Self { index, price }
    }

}

/// You are given an integer array `prices` where `prices[i]` is the price of the `ith` item in a
/// shop.
///
/// There is a special discount for items in the shop. If you buy the `ith` item, then you will
/// receive a discount equivalent to `prices[j]` where `j` is the minimum index such that `j > i`
/// and `prices[j] <= prices[i]`. Otherwise, you will not receive any discount at all.
///
/// Return an integer array `answer` where `answer[i]` is the final price you will pay for the
/// `ith` item of the shop, considering the special discount.
struct Solution;

impl Solution {

    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut result = prices.clone();

        let mut stack: Vec<Item> = Vec::new();
        for i in 0..n {
            let price = prices[i];
            let item = Item::new(i, price);

            while !stack.is_empty() {
                let last = stack.last().copied().unwrap();
                if price <= last.price {
                    let discounted = last.price - price;
                    result[last.index] = discounted;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(item);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let prices = vec![8,4,6,2,3];
        let result = Solution::final_prices(prices);
        assert_eq!(result, vec![4,2,4,2,3]);
    }

    #[test]
    fn example_2() {
        let prices = vec![1,2,3,4,5];
        let result = Solution::final_prices(prices);
        assert_eq!(result, vec![1,2,3,4,5]);
    }

    #[test]
    fn example_3() {
        let prices = vec![10,1,1,6];
        let result = Solution::final_prices(prices);
        assert_eq!(result, vec![9,0,1,6]);
    }

}

