/// You are given an `m x n` integer grid `accounts` where `accounts[i][j]` is
/// the amount of money the `ith` customer has in the `jth` bank. Return the
/// wealth that the richest customer has.
///
/// A customer's wealth is the amount of money they have in all their bank
/// accounts. The richest customer is the customer that has the maximum wealth.
struct Solution;

impl Solution {

    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap_or_default()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let accounts = vec![vec![1,2,3], vec![3,2,1]];
        let result = Solution::maximum_wealth(accounts);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let accounts = vec![vec![1,5], vec![7,3], vec![3,5]];
        let result = Solution::maximum_wealth(accounts);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_3() {
        let accounts = vec![vec![2,8,7], vec![7,1,3], vec![1,9,5]];
        let result = Solution::maximum_wealth(accounts);
        assert_eq!(result, 17);
    }

}
