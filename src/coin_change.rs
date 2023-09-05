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
        let n = amount as usize;
        let mut dp = vec![i32::MAX; n+1];
        dp[0] = 0;

        for i in 0..=n {
            if dp[i] < i32::MAX {
                let count = dp[i] + 1;

                for coin in &coins {
                    let current = i + (*coin as usize);
                    if current <= n {
                        dp[current] = dp[current].min(count);
                    }
                }
            }
        }

        let mut result = dp[n];
        if result == i32::MAX {
            result = -1;
        }
        result
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
