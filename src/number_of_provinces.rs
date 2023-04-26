use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

/// There are `n` cities. Some of them are connected, while some are not. If
/// city `a` is connected directly with city `b`, and city `b` is connected
/// directly with city `c`, then city `a` is connected indirectly with city
/// `c`.
///
/// A province is a group of directly or indirectly connected cities and no
/// other cities outside of the group.
///
/// You are given an `n x n` matrix `isConnected` where
/// `isConnected[i][j] = 1` if the `ith` city and the `jth` city are
/// directly connected, and `isConnected[i][j] = 0` otherwise.
///
/// Return the total number of provinces.
struct Solution;

impl Solution {

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

        let n = is_connected.len();
        for i in 0..n {
            let set = HashSet::new();
            graph.insert(i, set);
        }

        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 {
                    graph.entry(i)
                        .and_modify(|e| { e.insert(j); });
                    graph.entry(j)
                        .and_modify(|e| { e.insert(i); });
                }
            }
        }

        let mut seen: HashSet<usize> = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = 0;

        for i in 0..n {
            if !seen.contains(&i) {
                seen.insert(i);
                result += 1;
                queue.push_back(i);

                while !queue.is_empty() {
                    let j = queue.pop_front().unwrap();
                    let neighbors = &graph[&j];
                    for n in neighbors {
                        if !seen.contains(&n) {
                            seen.insert(*n);
                            queue.push_back(*n);
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
        let is_connected = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let is_connected = vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 3);
    }

}
