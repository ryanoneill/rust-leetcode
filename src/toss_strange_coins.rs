use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    index: usize,
    remaining: usize,
}

impl State {

    pub fn new(index: usize, remaining: usize) -> Self {
        Self { index, remaining }
    }

    pub fn got_heads(&self) -> Self {
        Self::new(self.index + 1, self.remaining - 1)
    }

    pub fn got_tails(&self) -> Self {
        Self::new(self.index + 1, self.remaining)
    }

}

/// You have some coins. The `i`-th coin has a probability `prob[i]` of facing
/// heads when tossed.
///
/// Return the probability that the number of coins facing heads equals
/// `target` if you toss every coin exactly once.
struct Solution;

impl Solution {

    pub fn worker(saved: &mut HashMap<State, f64>, prob: &Vec<f64>, state: State) -> f64 {
        if saved.contains_key(&state) {
            saved[&state]
        } else {
            let n = prob.len();

            let coin_prob = prob[state.index];

            let head_prob;
            if state.remaining == 0 {
                head_prob = 0.0;
            } else if state.index == n - 1 {
                head_prob = coin_prob;
            } else {
                head_prob = coin_prob * Self::worker(saved, prob, state.got_heads());
            }

            let tail_prob;
            if state.remaining >= (n - state.index) {
                tail_prob = 0.0;
            } else if state.index == n - 1 {
                tail_prob = 1.0 - coin_prob;
            } else {
                tail_prob = (1.0 - coin_prob) * Self::worker(saved, prob, state.got_tails());
            }

            let result = head_prob + tail_prob;
            saved.insert(state, result);
            result
        }
    }

    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let mut saved = HashMap::new();
        let state = State::new(0, target as usize);
        Self::worker(&mut saved, &prob, state)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let prob = vec![0.4];
        let target = 1;
        let result = Solution::probability_of_heads(prob, target);
        assert_eq!(result, 0.4);
    }

    #[test]
    fn example_2() {
        let prob = vec![0.5,0.5,0.5,0.5,0.5];
        let target = 0;
        let result = Solution::probability_of_heads(prob, target);
        assert_eq!(result, 0.03125);
    }

}
