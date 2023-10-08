use std::collections::HashMap;

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

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    index: usize,
    total: i32,
}

impl State {

    pub fn new() -> Self {
        Self { index: 0, total: 0 }
    }

    pub fn with(&self, stone: i32) -> Self {
        Self {
            index: self.index + 1,
            total: self.total + stone,
        }
    }

    pub fn without(&self) -> Self {
        Self {
            index: self.index + 1,
            total: self.total,
        }
    }

}

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, stones: &Vec<i32>, state: State, sum: i32, target: i32) -> i32 {
        let result;
        let n = stones.len();

        if results.contains_key(&state) {
            result = results[&state]
        } else {
            if state.total >= target || state.index >= n {
                result = (state.total - (sum - state.total)).abs();
            } else {
                let stone = stones[state.index];
                let with_state = state.with(stone);
                let without_state = state.without();

                let with_result = Self::worker(results, stones, with_state, sum, target);
                let without_result = Self::worker(results, stones, without_state, sum, target);
                result = with_result.min(without_result);
            }
            results.insert(state, result);
        }

        result
    }

    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum();
        let target = (sum as f64 / 2.0).ceil() as i32;

        let mut results = HashMap::new();
        let initial_state = State::new();

        let result = Self::worker(&mut results, &stones, initial_state, sum, target);
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
