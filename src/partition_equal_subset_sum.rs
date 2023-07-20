use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    index: usize,
    target: i64,
}

impl State {

    fn new(index: usize, target: i64) -> Self {
        Self { index, target }
    }

    fn with(&self, value: i64) -> Self {
        Self::new(self.index + 1, self.target - value)
    }

    fn without(&self) -> Self {
        Self::new(self.index + 1, self.target)
    }

}

/// Given an integer array `nums`, return `true` if you can partition the array into two subsets
/// such that the sum of the elements in both subsets is equaled or `false` otherwise.
struct Solution;

impl Solution {

    fn calculate_sum(nums: &Vec<i32>) -> i64 {
        let mut result = 0;
        
        for &num in nums {
            result += num as i64;
        }

        result
    }

    fn worker(results: &mut HashMap<State, bool>, nums: &Vec<i32>, state: State) -> bool {
        let mut result = false;

        if results.contains_key(&state) {
            results[&state]
        } else {
            let n = nums.len();
            let value = nums[state.index] as i64;

            if state.index == n-1 {
                if value == state.target {
                    result = true;
                } else if value == 0 {
                    result = true;
                } else {
                    result = false;
                }
            } else {
                result = result || Self::worker(results, nums, state.with(value));
                result = result || Self::worker(results, nums, state.without());
            }

            results.insert(state, result);

            result
        }
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut result = false;

        let sum = Self::calculate_sum(&nums);
        if sum % 2 == 0 {
            let target = sum / 2;
            let initial_state = State::new(0, target);
            let mut results = HashMap::new();
            result = Self::worker(&mut results, &nums, initial_state);
        } 

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,5,11,5];
        let result = Solution::can_partition(nums);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,5];
        let result = Solution::can_partition(nums);
        assert!(!result);
    }

}
