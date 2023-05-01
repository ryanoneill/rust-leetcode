use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

/// You are given an integer `n`, the number of nodes in a directed graph
/// where nodes are labeled from `0` to `n-1`. Each edge is red or blue in
/// this graph, and there could be self-edges and parallel edges.
///
/// You are given two arrays `redEdges` and `blueEdges` where:
///
/// * `redEdges[i] = [ai, bi]` indicates that there is a directed red edge
///   from node `ai` to node `bi` in the graph, and
///
/// * `blueEdges[j] = [uj, vj]` indicates that there is a directed blue edge
///   from node `uj` to node `vj` in the graph.
///
/// Return an array `answer` of length `n` where each `answer[x]` is the length
/// of the shortest path from node `0` to node `x` such that the edge colors
/// alternate along the path, or `-1` if such a path does not exist.

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Blue,
}

impl Color {

    fn flip(&self) -> Self {
        if *self == Color::Red { Color::Blue }
        else { Color::Red }
    }

}

#[derive(Eq, Hash, PartialEq)]
struct State {
    x: i32,
    color: Color,
    steps: i32,
}

impl State {

    fn new(x: i32, color: Color, steps: i32) -> Self {
        Self { x, color, steps }
    }

    fn from_old(x: i32, state: &Self) -> Self {
        Self::new(x, state.color.flip(), state.steps + 1)
    }

    fn is_blue(&self) -> bool {
        self.color == Color::Blue
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn key(&self) -> (i32, Color) {
        (self.x, self.color)
    }

}

struct Solution;

impl Solution {

    fn to_map(edges: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
        let mut result = HashMap::new();
        for edge in edges {
            result.entry(edge[0])
                .or_insert(HashSet::new())
                .insert(edge[1]);
        }
        result
    }

    fn get_edges(
        state: &State,
        reds: &HashMap<i32, HashSet<i32>>,
        blues: &HashMap<i32, HashSet<i32>>
    ) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if state.is_red() && blues.contains_key(&state.x) {
            result = Vec::from_iter(blues[&state.x].iter().copied());
        } else if state.is_blue() && reds.contains_key(&state.x) {
            result = Vec::from_iter(reds[&state.x].iter().copied());
        }

        result
    }

    pub fn shortest_alternating_path(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>
    ) -> Vec<i32> {
        let reds = Self::to_map(red_edges);
        let blues = Self::to_map(blue_edges);

        let mut result = vec![-1; n as usize];
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        let red_start_state = State::new(0, Color::Red, 0);
        let blue_start_state = State::new(0, Color::Blue, 0);
        result[0] = 0;

        seen.insert(red_start_state.key());
        seen.insert(blue_start_state.key());
        queue.push_back(red_start_state);
        queue.push_back(blue_start_state);

        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            let edges = Self::get_edges(&state, &reds, &blues);
            for edge in edges {
                let state = State::from_old(edge, &state);
                if !seen.contains(&state.key()) {
                    if result[state.x as usize] == -1 {
                        result[state.x as usize] = state.steps;
                    }
                    seen.insert(state.key());
                    queue.push_back(state);
                }

            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let red_edges = vec![vec![0,1], vec![1,2]];
        let blue_edges = vec![];
        let result = Solution::shortest_alternating_path(n, red_edges, blue_edges);
        assert_eq!(result, vec![0,1,-1]);
        // 0 -> RED! -> 1 ... X ... 2
    }

    #[test]
    fn example_2() {
        let n = 3;
        let red_edges = vec![vec![0,1]];
        let blue_edges = vec![vec![2,1]];
        let result = Solution::shortest_alternating_path(n, red_edges, blue_edges);
        assert_eq!(result, vec![0,1,-1]);
        // 0 -> RED! -> 1 ... X ... 2
    }

    #[test]
    fn own_test() {
        let n = 5;
        let red_edges = vec![vec![0,1], vec![1,2], vec![3,4]];
        let blue_edges = vec![vec![1,1], vec![2,3]];
        let result = Solution::shortest_alternating_path(n, red_edges, blue_edges);
        assert_eq!(result, vec![0,1,3,4,5]);
        // 0 -> RED! -> 1 -> BLUE! -> 1 -> RED! -> 2 -> BLUE! -> 3 -> RED! -> 4
    }

}
