use crate::union_find::UnionFind;

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

    /// Switched from DFS/BFS to UnionFind because the graph is undirected and
    /// the question doesn't ask what the path is between source and
    /// destination, but only whether one exists.
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            uf.union(x, y);
        }
        uf.connected(source, destination)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        let result = Solution::valid_path(n, edges, source, destination);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
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
