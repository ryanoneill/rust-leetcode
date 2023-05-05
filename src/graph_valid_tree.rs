use crate::union_find::UnionFind;

/// You have a graph of `n` nodes labeled from `0` to `n - 1`. You are given
/// an integer n and a list of `edges` where `edges[i] = [ai, bi]` indicates
/// that there is an undirected edge between nodes `ai` and `bi` in the graph.
///
/// Return `true` if the edges of the given graph make up a valid tree, and
/// `false` otherwise.
struct Solution;

impl Solution {

    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        let mut result = true;
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            if uf.connected(x, y) {
                result = false;
                break;
            } else {
                uf.union(x, y);
            }
        }
        result && uf.roots().len() == 1
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn example_1() {
        let n = 5;
        let edges = vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]];
        let result = Solution::valid_tree(n, edges);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let edges = vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]];
        let result = Solution::valid_tree(n, edges);
        assert!(!result);
    }

}
