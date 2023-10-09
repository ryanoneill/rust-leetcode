use crate::union_find::UnionFind;

/// In this problem, a tree is an undirected graph that is connected and has no cycles.
///
/// You are given a graph that started as a tree with `n` nodes labeled from `1` to `n`,
/// with one additional edge added. The added edge has two different vertices chosen from `1` to
/// `n`, and was not an edge that already existed. The graph is represented as an array `edges` of
/// length `n` where `edges[i] = [ai, bi]` indicates that there is an edge between nodes `ai` and
/// `bi` in the graph.
///
/// Return an edge that can be removed so that the resulting graph is a tree of `n` nodes. If there
/// are multiple answers, return the answer that occurs last in the input.
struct Solution;

impl Solution {

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0,0];
        let n = edges.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            let edge = &edges[i];
            let a = edge[0] - 1;
            let b = edge[1] - 1;

            if uf.connected(a, b) {
                result[0] = edge[0];
                result[1] = edge[1];
                break;
            } else {
                uf.union(a, b);
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
        let edges = vec![vec![1,2], vec![1,3], vec![2,3]];
        let result = Solution::find_redundant_connection(edges);
        assert_eq!(result, vec![2,3]);
    }

    #[test]
    fn example_2() {
        let edges = vec![vec![1,2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]];
        let result = Solution::find_redundant_connection(edges);
        assert_eq!(result, vec![1,4]);
    }

}

// 1. Write down the problem ✓
// 2. Clarify the problem space ✓
// ** Nodes n labeled from 1 to n
// ** Edges between node a and b
// ** One extra edge that is redundant
// ** Return that edge
// ** n >= 3 and n <= 1000
// ** no repeated edges
// ** graph is connected
// ** n should be the number of edges since (n-1) needed to connect n nodes plus one extra
// ** Input: List of pairs of nodes
// ** Output: Single pair of nodes
//
// 3. Write down the test cases ✓
// ** Input: [[1,2], [1,3], [2,3]]
// ** Output: [2,3]
//
// ** Input: [[1,2],[2,3],[3,4],[1,4],[1,5]]
// ** Output: [1,4]
//
// 4. Describe and write down the algorithm
// ** let n = edges.len()
// ** Set up Union Find data structure of size n
// ** for each edge, check whether a-1 and b-1 are connected
// **   if so, then this is the edge that is superfluous, set it to the result and exit
// **   if not, then connect a-1 and b-1 in the uf data structure.
// ** Time Complexity: O(n) 
// ** Space Compexlity: O(n)
//
// 5. Start Coding
