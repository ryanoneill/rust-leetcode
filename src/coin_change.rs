use std::cmp::min;
use std::collections::HashMap;

/// You are given an integer array `coins` representing coins of different
/// denominations and an integer `amount` representing a total amount of money.
///
/// Return the fewest number of coins that you need to make up that amount. If
/// that amount of money cannot be made up by any combination of the coins,
/// return `-1`.
///
/// You may assume that you have an infinite number of each kind of coin.
struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut mins = HashMap::new();
        Self::worker(&coins, amount, &mut mins)
    }

    fn worker(coins: &Vec<i32>, amount: i32, mins: &mut HashMap<i32, i32>) -> i32 {
        if amount < 0 {
            -1
        } else if amount == 0 {
            0
        } else if mins.contains_key(&amount) {
            mins[&amount]
        } else {
            let mut result = -1;
            for coin in coins {
                let sub = Self::worker(coins, amount - coin, mins);
                if sub != -1 {
                    let current = sub + 1;
                    if result == -1 {
                        result = current;
                    } else {
                        result = min(result, current);
                    }
                }
            }

            mins.insert(amount, result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let result = Solution::coin_change(coins, amount);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let coins = vec![2];
        let amount = 3;
        let result = Solution::coin_change(coins, amount);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let coins = vec![1];
        let amount = 0;
        let result = Solution::coin_change(coins, amount);
        assert_eq!(result, 0);
    }
}
