use crate::adjacency_graph::AdjacencyGraph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// A tree is an undirected graph in which any two vertices are connected by
/// exactly one path. In other words, any connected graph without simple cycles
/// is a tree.
///
/// Given a tree of `n` nodes labelled from `0` to `n - 1`, and an array of
/// `n - 1` `edges` where `edges[i] = [ai, bi]` indicates that there is an
/// undirected edge between the two nodes `ai` and `bi` in the tree, you can
/// choose any node of the tree as the root. When you select a node `x` as the
/// root, the result tree has height `h`. Among all possible rooted trees,
/// those with minimum height (i.e. `min(h)`) are called minimum height trees
/// (MHTs).
///
/// Return a list of all MHT's root labels. You can return the answer in any
/// order.
///
/// The height of a rooted tree is the number of edges on the longest downward
/// path between the root and a leaf.
struct Solution;

impl Solution {

    pub fn find_min_height_tree(_n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let graph = AdjacencyGraph::from_vec(&edges);
        graph.find_centroids()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let edges = vec![vec![1,0], vec![1,2], vec![1,3]];
        let result = Solution::find_min_height_tree(n, edges);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let edges = vec![vec![3,0], vec![3,1], vec![3,2], vec![3,4], vec![5,4]];
        let mut result = Solution::find_min_height_tree(n, edges);
        result.sort();
        assert_eq!(result, vec![3,4]);
    }

}
