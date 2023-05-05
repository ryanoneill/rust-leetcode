use crate::union_find::UnionFind;

/// You have a graph of `n` nodes. You are given an integer `n` and an array
/// `edges` where `edges[i] = [ai, bi]` indicates that there is an edge between
/// `ai` and `bi` in the graph.
///
/// Return the number of connected components in the graph.
struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            uf.union(x, y);
        }

        uf.roots().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
        let result = Solution::count_components(n, edges);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = Solution::count_components(n, edges);
        assert_eq!(result, 1);
    }
}
