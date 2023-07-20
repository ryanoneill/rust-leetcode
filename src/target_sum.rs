use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    index: usize,
    target: i32
}

impl State {

    pub fn new(index: usize, target: i32) -> Self {
        Self { index, target }
    }

    pub fn plus(&self, value: i32) -> Self {
        Self::new(self.index + 1, self.target + value)
    }

    pub fn minus(&self, value: i32) -> Self {
        Self::new(self.index + 1, self.target - value)
    }

}

/// You are given an integer array `nums` and an integer `target`.
///
/// You want to build an expression out of nums by adding one of the symbols `'+'` and `'-'` before
/// each integer in nums and then concatenate all the integers.
///
/// * For example, if `nums = [2,1]`, you can add a `'+'` before `2` and a `'-'` before `1` and
///   concatenate them to build the expression `"+2-1"`.
///
/// Return the number of different expressions that you can build, which evaluates to `target`.
struct Solution;

impl Solution {

    fn worker(results: &mut HashMap<State, i32>, nums: &Vec<i32>, n: usize, state: State) -> i32 {
        if results.contains_key(&state) {
            results[&state]
        } else {
            let mut result = 0;

            let value = nums[state.index];
            if state.index == n-1 {
                if value == state.target {
                    result += 1;
                }
                if value == (-1 * state.target) {
                    result += 1;
                }
            } else {
                let plus_state = state.plus(value);
                result += Self::worker(results, nums, n, plus_state);

                let minus_state = state.minus(value);
                result += Self::worker(results, nums, n, minus_state);
            }
            results.insert(state, result);

            result
        }
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let initial_state = State::new(0, target);
        let mut results = HashMap::new();
        Self::worker(&mut results, &nums, n, initial_state)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,1,1,1,1];
        let target = 3;
        let result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let target = 1;
        let result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let nums = vec![1,0];
        let target = 1;
        let result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, 2);
    }

}
