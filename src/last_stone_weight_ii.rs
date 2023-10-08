use std::collections::HashSet;

/// You are given an array of integers `stones` where `stones[i]` is the weight of the `ith` stone.
///
/// We are playing a game with the stones. On each turn, we choose any two stones and smash them
/// together. Suppose the stones have weights `x` and `y` with `x <= y`. The result of the smash
/// is:
///
/// * If `x == y`, both stones are destroyed, and
///
/// * If `x != y`, the stone of weight `x` is destroyed, and the stone of weight `y` has new weight
///   `y - x`.
///
/// At the end of the game, there is at most one stone left.
///
/// Return the smallest possible weight of the left stone. If there are no stones left, return `0`.
struct Solution;

impl Solution {

    // Hint 1 says: Think of the final answer as a sum of weights with + or - sign
    // symbols in front of each weight.
    //
    // The solution to this problem is based on the difference between stones in 
    // pile A and stones in pile B. A stone can either be included in pile A 
    // (plus sign) or included in pile B (minus sign). 
    //
    // A high positive value indicates that many stones are in pile A.
    // A low  negative value indicates that many stones are in pile B.
    //
    // We want the solution that is 0 or closest to it.
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut weights = HashSet::new();

        let stone = stones[0];
        weights.insert(stone);
        weights.insert(-stone);

        for i in 1..n {
            let stone = stones[i];
            let mut next = HashSet::new();

            for weight in weights {
                next.insert(weight + stone);
                next.insert(weight - stone);
            }
            weights = next;
        }

        let mut result = i32::MAX;
        for weight in weights {
            result = result.min(weight.abs());
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[test]
    fn example_1() {
        let stones = vec![2,7,4,1,8,1];
        let result = Solution::last_stone_weight_ii(stones);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let stones = vec![31,26,33,21,40];
        let result = Solution::last_stone_weight_ii(stones);
        assert_eq!(result, 5);
    }

}
