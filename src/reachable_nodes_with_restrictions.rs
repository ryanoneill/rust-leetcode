use std::collections::HashMap;
use std::collections::HashSet;

/// There is an undirected tree with `n` nodes labeled from `0` to `n-1` and
/// `n-1` edges.
///
/// You are given a 2D integer array `edges` of length `n-1` where
/// `edges[i] = [ai, bi]` indicates that there is an edge between nodes `ai`
/// and `bi` in the tree. You are also given an integer array `restricted`
/// which represents restricted nodes.
///
/// Return the maximum number of nodes you can reach from node `0` without
/// visiting a restricted node.
///
/// Note that node `0` will not be a restricted node.
struct Solution;

impl Solution {

    fn make_adjacency_map(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> HashMap<i32, HashSet<i32>> {
        let mut adj = HashMap::new();
        let banned: HashSet<i32> = HashSet::from_iter(restricted);

        for i in 0..n {
            adj.insert(i as i32, HashSet::new());
        }
        for edge in edges {
            let a = edge[0];
            let b = edge[1];

            // If it's restricted, pretend like the edges to it just don't exist
            if !banned.contains(&a) && !banned.contains(&b) {
                adj.entry(a).and_modify(|c| { c.insert(b); });
                adj.entry(b).and_modify(|c| { c.insert(a); });
            }
        }

        adj
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut result = 0;

        if n > 0 {
            let mut seen = HashSet::new();
            let mut stack = Vec::new();
            let adj = Self::make_adjacency_map(n, edges, restricted);

            stack.push(0);
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if !seen.contains(&current) {
                    result += 1;
                    seen.insert(current);
                    let connected = &adj[&current];
                    for conn in connected {
                        stack.push(*conn);
                    }
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
        let n = 7;
        let edges = vec![vec![0,1], vec![1,2], vec![3,1], vec![4,0], vec![0,5], vec![5,6]];
        let restricted = vec![4,5];
        let result = Solution::reachable_nodes(n, edges, restricted);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let n = 7;
        let edges = vec![vec![0,1], vec![0,2], vec![0,5], vec![0,4], vec![3,2], vec![6,5]];
        let restricted = vec![4,2,1];
        let result = Solution::reachable_nodes(n, edges, restricted);
        assert_eq!(result, 3);
    }

}
