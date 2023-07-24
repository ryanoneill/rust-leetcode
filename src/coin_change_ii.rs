use std::collections::HashMap;

/// You are given an integer array `coins` representing a total amount of money.
///
/// Return the number of combinations that make up that amount. If that amount of money cannot be
/// made up by any combination of the coins, return `0`.
///
/// You may assume that you have an infinite number of each kind of coin.
///
/// The answer is guaranteed to fit into a signed 32-bit integer.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<(usize, i32), i32>, coins: &Vec<i32>, amount: i32, i: usize) -> i32 {
        let n = coins.len();
        let coin = coins[i];
        let mut result = 0;
        let key = (i, amount);

        if amount == 0 {
            1
        } else if results.contains_key(&key) {
            results[&key]
        } else {
            if i == n - 1 {
                if amount % coin == 0 {
                    result = 1;
                } 
            } else {
                result += Self::worker(results, coins, amount, i + 1);
                let mut current = amount;
                current -= coin;
                while current >= 0 {
                    result += Self::worker(results, coins, current, i + 1);
                    current -= coin;
                }
            }

            results.insert(key, result);

            result
        }
    }

    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut results = HashMap::new();
        Self::worker(&mut results, &coins, amount, 0)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let amount = 5;
        let coins = vec![1,2,5];
        let result = Solution::change(amount, coins);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let amount = 3;
        let coins = vec![2];
        let result = Solution::change(amount, coins);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let amount = 10;
        let coins = vec![10];
        let result = Solution::change(amount, coins);
        assert_eq!(result, 1);
    }

}
