use std::collections::VecDeque;

/// You have a graph of `n` nodes. You are given an integer `n` and an array
/// `edges` where `edges[i] = [ai, bi]` indicates that there is an edge between
/// `ai` and `bi` in the graph.
///
/// Return the number of connected components in the graph.
struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut connected: Vec<Vec<usize>> = vec![vec![]; n];
        let mut result = 0;
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            connected[from].push(to);
            connected[to].push(from);
        }

        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            if !seen[i] {
                result += 1;
                queue.push_back(i);
                while !queue.is_empty() {
                    let item = queue.pop_front().unwrap();
                    if !seen[item] {
                        seen[item] = true;
                        let outs = &connected[item];
                        for out in outs {
                            queue.push_back(*out);
                        }
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
