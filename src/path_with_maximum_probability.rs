use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Item {
    probability: f64,
    destination: i32,
}

impl Item {

    fn new(probability: f64, destination: i32) -> Self {
        Self { probability, destination }
    }

}

impl Eq for Item {}
impl Ord for Item {

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }

}

/// You are given an undirected weighted graph of `n` nodes (0-indexed), represented by an edge
/// list where `edges[i] = [a, b]` is an undirected edge connecting the nodes `a` and `b` with a
/// probability of success traversing that edge `succProb[i]`.
///
/// Given two nodes `start` and `end`, find the path with the maximum probability of success to go
/// from `start` to `end` and return its success probability.
///
/// If there is no path from `start` to `end`, return 0.
struct Solution;

impl Solution {

    fn to_adjacency_map(edges: Vec<Vec<i32>>, succ_prob: Vec<f64>) -> HashMap<i32, HashMap<i32, f64>> {
        let mut result = HashMap::new();

        let n = edges.len();
        for i in 0..n {
            let edge = &edges[i];
            let a = edge[0];
            let b = edge[1];
            let probability = succ_prob[i];

            result
                .entry(a)
                .or_insert(HashMap::new())
                .insert(b, probability);

            result
                .entry(b)
                .or_insert(HashMap::new())
                .insert(a, probability);
        }

        result
    }

    pub fn max_probability(_n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let probabilities = Self::to_adjacency_map(edges, succ_prob);

        let mut maxes: HashMap<i32, f64> = HashMap::new();
        let mut reachable = BinaryHeap::new();

        let item = Item::new(1.0, start_node);
        reachable.push(item);

        while !reachable.is_empty() {
            let item = reachable.pop().unwrap();
            if !maxes.contains_key(&item.destination) {
                maxes.insert(item.destination, item.probability);
                if item.destination == end_node {
                    break;
                } else if probabilities.contains_key(&item.destination) {
                    for (neighbor, prob) in &probabilities[&item.destination] {
                        if !maxes.contains_key(&neighbor) {
                            let total_prob = item.probability * prob;
                            let item = Item::new(total_prob, *neighbor);
                            reachable.push(item);
                        }
                    }
                }
            }

        }

        if maxes.contains_key(&end_node) {
            maxes[&end_node]
        } else {
            0.0
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let edges = vec![vec![0,1], vec![1,2], vec![0,2]];
        let succ_prob = vec![0.5,0.5,0.2];
        let start = 0;
        let end = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start, end);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let edges = vec![vec![0,1], vec![1,2], vec![0,2]];
        let succ_prob = vec![0.5,0.5,0.3];
        let start = 0;
        let end = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start, end);
        assert_eq!(result, 0.30);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let edges = vec![vec![0,1]];
        let succ_prob = vec![0.5];
        let start = 0;
        let end = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start, end);
        assert_eq!(result, 0.00);
    }

}

