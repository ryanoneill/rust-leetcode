use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Connection {
    from: usize,
    to: usize,
}

impl Connection {

    pub fn new(from: usize, to: usize) -> Self {
        Connection { from, to }
    }

}

/// There are `n` cities numbered from `0` to `n - 1` and `n - 1` roads such
/// that there is only one way to travel between two cities (this network form
/// a tree). Last year, the ministry of transport decided to orient the roads
/// in one direction because they are too narrow.
///
/// Roads are represented by `connections` where `connections[i] = [ai, bi]`
/// represents a road from city `ai` to city `bi`.
///
/// This year, there will be a big event in the capital (city `0`) and many
/// people want to travel to this city.
///
/// Your task consists of reorienting some roads such that each city can visit
/// the city `0`. Return the minimum number of edges changed.
///
/// It's guaranteed that each city can reach city `0` after reorder.
struct Solution;

impl Solution {

    pub fn min_reorder(_n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut seen = HashSet::new();

        let mut adjacency = HashMap::new();
        for item in connections {
            let from = item[0] as usize;
            let to = item[1] as usize;
            let connection = Connection::new(from, to);
            adjacency
                .entry(from)
                .or_insert(HashSet::new())
                .insert(connection);
            adjacency
                .entry(to)
                .or_insert(HashSet::new())
                .insert(connection);
        }

        let mut queue = VecDeque::new();
        queue.push_back(0);
        seen.insert(0);
        while !queue.is_empty() {
            let value = queue.pop_front().unwrap();
            if adjacency.contains_key(&value) {
                let conns = &adjacency[&value];
                for conn in conns {
                    if conn.to == value {
                        if !seen.contains(&conn.from) {
                            seen.insert(conn.from);
                            queue.push_back(conn.from);
                        }
                    } else {
                        if !seen.contains(&conn.to) {
                            seen.insert(conn.to);
                            queue.push_back(conn.to);
                            // Changed direction of this road
                            result += 1;
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
        let n = 6;
        let connections = vec![vec![0,1], vec![1,3], vec![2,3], vec![4,0], vec![4,5]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let n = 5;
        let connections = vec![vec![1,0], vec![1,2], vec![3,2], vec![3,4]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let n = 3;
        let connections = vec![vec![1,0], vec![2,0]];
        let result = Solution::min_reorder(n, connections);
        assert_eq!(result, 0);
    }

}
