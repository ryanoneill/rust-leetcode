/// You are given an array of integers `stones` where `stones[i]` is the weight of the `ith` stone.
///
/// We are playing a game with the stones. On each turn, we choose any two stones and smash them
/// together. Suppose the stones have weights `x` and `y` with `x <= y`. The result of the smash
/// is:
///
/// * If `x == y`, both stones are destroyed, and
///
/// * If `x != y`, the stone of weight `x` is destroyed.
///
/// At the end of the game, there is at most one stone left.
///
/// Return the smallest possible weight of the left stone. If there are no stones left, return `0`.
struct Solution;

impl Solution {

    pub fn last_stone_weight_ii(_stones: Vec<i32>) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[ignore]
    #[test]
    fn example_1() {
        let stones = vec![2,7,4,1,8,1];
        let result = Solution::last_stone_weight_ii(stones);
        assert_eq!(result, 1);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let stones = vec![31,26,33,21,40];
        let result = Solution::last_stone_weight_ii(stones);
        assert_eq!(result, 5);
    }

}
