use crate::union_find::UnionFind;
use std::collections::HashSet;

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
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in i..n {
                if is_connected[i][j] == 1 {
                    uf.union(i as i32, j as i32);
                }
            }
        }

        uf.count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let is_connected = vec![vec![1,0,0,1], vec![0,1,1,0], vec![0,1,1,1], vec![1,0,1,1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_4() {
        let is_connected = vec![
            vec![1,1,0,0,0,0,0,1,0,0,0,0,0,0,0],
            vec![1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
            vec![0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
            vec![0,0,0,1,0,1,1,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,1,0,0,0,0,1,1,0,0,0,0],
            vec![0,0,0,1,0,1,0,0,0,0,1,0,0,0,0],
            vec![0,0,0,1,0,0,1,0,1,0,0,0,0,1,0],
            vec![1,0,0,0,0,0,0,1,1,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,1,1,1,0,0,0,0,1,0],
            vec![0,0,0,0,1,0,0,0,0,1,0,1,0,0,1],
            vec![0,0,0,0,1,1,0,0,0,0,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,1,0,1,0,0,0,0,1,0],
            vec![0,0,0,0,0,0,0,0,0,1,0,0,0,0,1]];
        let result = Solution::find_circle_num(is_connected);
        assert_eq!(result, 3);
    }

}
