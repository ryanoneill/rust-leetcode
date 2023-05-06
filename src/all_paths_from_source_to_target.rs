use std::clone;

/// Given a directed acyclic graph (DAG) of `n` nodes labeled from `0` to
/// `n -1`, find all possible paths from node `0` to node `n - 1` and return
/// them in any order.
///
/// The graph is given as follows: `graph[i]` is a list of all nodes you can
/// visit from node `i` (i.e., there is a directed edge from node `i` to
/// node `graph[i][j]`).
struct Solution;

impl Solution {

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        Self::worker(&graph, 0, vec![], &mut results);
        results
    }

    fn worker(graph: &Vec<Vec<i32>>, node: i32, path: Vec<i32>, results: &mut Vec<Vec<i32>>) {
        let n = graph.len();
        let i = node as usize;
        let mut path = path;
        path.push(node);
        if i == n - 1 {
            results.push(path);
        } else {
            for &next in &graph[i] {
                let cloned_path = path.clone();
                Self::worker(graph, next, cloned_path, results);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let graph = vec![vec![1,2], vec![3], vec![3], vec![]];
        let mut result = Solution::all_paths_source_target(graph);
        result.sort();
        assert_eq!(result, vec![vec![0,1,3], vec![0,2,3]]);
    }

    #[test]
    fn example_2() {
        let graph = vec![vec![4,3,1], vec![3,2,4], vec![3], vec![4], vec![]];
        let mut result = Solution::all_paths_source_target(graph);
        result.sort();
        assert_eq!(result, vec![vec![0,1,2,3,4], vec![0,1,3,4], vec![0,1,4], vec![0,3,4], vec![0,4]]);
    }

}
