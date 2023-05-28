use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct State {
    i: usize,
    left: usize,
}

impl State {

    pub fn new(i: usize, left: usize) -> Self {
        Self { i, left }
    }

    pub fn take_left(&self) -> Self {
        Self::new(self.i + 1, self.left + 1)
    }

    pub fn take_right(&self) -> Self {
        Self::new(self.i + 1, self.left)
    }

    pub fn get_left(&self, nums: &Vec<i32>) -> i32 {
        nums[self.left]
    }

    pub fn get_right(&self, nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let right = n - 1 - (self.i - self.left);
        nums[right]
    }

}

/// You are given two 0-indexed integer arrays `nums` and `multipliers` of size
/// `n` and `m` respectively, where `n >= m`.
///
/// You begin with a score of `0`. You want to perform exactly `m` operations.
/// On the `ith` operation (0-indexed) you will:
///
/// * Choose one integer `x` from either the start or the end of the array
///   `nums`.
///
/// * Add `multipliers[i] * x` to your score.
///   * Note that `multipliers[0]` corresponds to the fist operation,
///     `mutlipliers[1]` to the second operation, and so on.
///
/// * Remove `x` from `nums`. 
///
/// Return the maximum score after performing `m` operations.
struct Solution;

impl Solution {

    fn worker(nums: &Vec<i32>, multipliers: &Vec<i32>, saved: &mut HashMap<State, i32>, state: State) -> i32 {
        let m = multipliers.len();
        if state.i == m {
            0
        } else if saved.contains_key(&state) {
            saved[&state]
        } else {
            let mult = multipliers[state.i];
            let left = state.get_left(nums) * mult;
            let right = state.get_right(nums) * mult;

            let from_left = left + Self::worker(nums, multipliers, saved, state.take_left());
            let from_right = right + Self::worker(nums, multipliers, saved, state.take_right());
            let result = from_left.max(from_right);
            saved.insert(state, result);
            result
        }
    }

    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut saved = HashMap::new();
        let initial = State::new(0, 0);
        Self::worker(&nums, &multipliers, &mut saved, initial)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3];
        let multipliers = vec![3,2,1];
        let result = Solution::maximum_score(nums, multipliers);
        assert_eq!(result, 14);
    }

    #[test]
    fn example_2() {
        let nums = vec![-5,-3,-3,-2,7,1];
        let multipliers = vec![-10,-5,3,4,6];
        let result = Solution::maximum_score(nums, multipliers);
        assert_eq!(result, 102);
    }

}
