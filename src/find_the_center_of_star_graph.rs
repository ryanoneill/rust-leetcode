/// There is an undirected star graph consisting of `n` nodes labeled from `1` to `n`. A star graph
/// is a graph where there is one center node and exactly `n-1` edges that connect the center node
/// with every other node.
///
/// You are given a 2D integer array `edges` where each `edges[i] = [ui, vi]` indicates that there
/// is an edge between the nodes `ui` and `vi`. Return the center of the given star graph.
struct Solution;

impl Solution {

    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let edges = vec![vec![1,2], vec![2,3], vec![4,2]];
        let result = Solution::find_center(edges);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let edges = vec![vec![1,2], vec![5,1], vec![1,3], vec![1,4]];
        let result = Solution::find_center(edges);
        assert_eq!(result, 1);
    }

}
