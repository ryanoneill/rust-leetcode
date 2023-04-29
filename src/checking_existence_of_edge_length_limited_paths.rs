use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// An undirected graph of `n` nodes is defined by `edgeList`, where
/// `edgeList[i] = [ui, vi, disi]` denotes an edge between nodes `ui` and `vi`
/// with distance `disi`. Note that there may be multiple edges between two
/// nodes.
///
/// Given an array `queries`, where `queries[j] = [pj, qj, limit]`, your task
/// is to determine for each `queries[j]` where there is a path between `pj`
/// and `qj` such that each edge on the path has a distance strictly less than
/// `limitj`.
///
/// Return a boolean array `answer`, where `answer.length == queries.length` and
/// the `jth` value of `answer` is `true` if there is a path for `queries[j]` is
/// `true`, and `false` otherwise.
struct Solution;

impl Solution {

    fn add_edge_if_min(edges: &mut HashMap<i32, HashMap<i32, i32>>, edge: Vec<i32>) {
        let a = edge[0];
        let b = edge[1];
        let distance = edge[2];

        match edges.get_mut(&a) {
            Some(map) => {
                map.entry(b)
                    .and_modify(|d| if distance < *d { *d = distance; })
                    .or_insert(distance);
            }
            None => {
                let mut edges_from = HashMap::new();
                edges_from.insert(b, distance);
                edges.insert(a, edges_from);
            }
        }

        match edges.get_mut(&b) {
            Some(map) => {
                map.entry(a)
                    .and_modify(|d| if distance < *d { *d = distance; })
                    .or_insert(distance);
            }
            None => {
                let mut edges_from = HashMap::new();
                edges_from.insert(a, distance);
                edges.insert(b, edges_from);
            }
        }

    }

    fn make_adj_map(edge_list: Vec<Vec<i32>>) -> HashMap<i32, HashMap<i32, i32>> {
        let mut result: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

        for edge in edge_list {
            Self::add_edge_if_min(&mut result, edge);
        }

        result
    }

    fn path_exists(adj: &HashMap<i32, HashMap<i32, i32>>, query: &Vec<i32>) -> bool {
        let mut result = false;
        let from = query[0];
        let to = query[1];
        let limit = query[2];

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(from);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if !seen.contains(&current) {
                seen.insert(current);
                if current == to {
                    result = true;
                    break;
                } else if adj.contains_key(&current) {
                    let edges_from = &adj[&current];
                    for (to, distance) in edges_from {
                        if *distance < limit {
                            queue.push_back(*to);
                        }
                    }
                }
            }
        }

        result
    }

    pub fn distance_limited_paths_exist(
        _: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>
    ) -> Vec<bool> {
        let adj = Self::make_adj_map(edge_list);
        queries.iter().map(|query| {
            Self::path_exists(&adj, query)
        }).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let edge_list = vec![vec![0,1,2], vec![1,2,4], vec![2,0,8], vec![1,0,16]];
        let queries = vec![vec![0,1,2], vec![0,2,5]];
        let result = Solution::distance_limited_paths_exist(n, edge_list, queries);
        assert_eq!(result, vec![false, true]);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let edge_list = vec![vec![0,1,10], vec![1,2,5], vec![2,3,9], vec![3,4,13]];
        let queries = vec![vec![0,4,14], vec![1,4,13]];
        let result = Solution::distance_limited_paths_exist(n, edge_list, queries);
        assert_eq!(result, vec![true, false]);
    }

}
