use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// There is a bi-directional graph with `n` vertices, where each vertex is
/// labeled from `0` to `n-1` (inclusive). The edges in the graph are
/// represented as a 2D integer array `edges`, where each `edges[i] = [ui, vi]`
/// denotes a bi-directional edge between vertex `ui` and vertex `vi`. Every
/// vertex pair is connected by at most one edge, and no vertex has an edge to
/// itself.
///
/// You want to determine if there is a valid path that exists from vertex
/// `source` to vertex `destination`.
///
/// Given `edges` and the integers `n`, `source`, and `destination`, return
/// `true` if there is a valid path from `source` to `desintation`, or `false`
/// otherwise.
struct Solution;

impl Solution {

    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut paths = HashMap::new();
        let needle = destination as usize;
        let n = n as usize;
        for i in 0..n {
            paths.insert(i, HashSet::new());
        }

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            paths.entry(a).and_modify(|p| { p.insert(b); });
            paths.entry(b).and_modify(|p| { p.insert(a); });
        }

        let mut result = false;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        if source == destination { result = true }
        else {
            queue.push_back(source as usize);
            while !queue.is_empty() {
                let item = queue.pop_front().unwrap();
                if !seen.contains(&item) {
                    seen.insert(item);
                    let outs = &paths[&item];
                    for o in outs.iter() {
                        if *o == needle {
                            result = true;
                            break;
                        } else { queue.push_back(*o); }
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
        let n = 3;
        let edges = vec![vec![0,1], vec![1,2], vec![2,0]];
        let source = 0;
        let destination = 2;
        let result = Solution::valid_path(n, edges, source, destination);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let edges = vec![vec![0,1], vec![0,2], vec![3,5], vec![5,4], vec![4,3]];
        let source = 0;
        let destination = 5;
        let result = Solution::valid_path(n, edges, source, destination);
        assert!(!result);
    }

    #[test]
    fn same() {
        let n = 1;
        let edges = vec![];
        let source = 0;
        let destination = 0;
        let result = Solution::valid_path(n, edges, source, destination);
        assert!(result);
    }

}
