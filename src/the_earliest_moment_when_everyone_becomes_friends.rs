use crate::union_find::UnionFind;

/// There are n people in a social group labeled from `0` to `n-1`. You are
/// given an array `logs` where `logs[i] = [timestampi, xi, yi]` indicates
/// that `xi` and `yi` will be friends at the time `timestampi`.
///
/// Friendship is symmetric. That means if `a` is friends with `b`, the `b` is
/// friends with `a`. Also, person `a` is acquainted with a person `b` if `a`
/// is friends with `b`, or `a` is a friend of someone acquainted with `b`.
///
/// Return the earliest time for which every person became acquanted with every
/// other person. If there is no such earliest time, return `-1`.
struct Solution;

impl Solution {

    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut logs = logs;
        logs.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut result = -1;

        let n = n as usize;
        let mut uf = UnionFind::new(n);

        for log in logs {
            let timestamp = log[0];
            let x = log[1];
            let y = log[2];
            uf.union(x, y);
            if uf.count() == 1 {
                result = timestamp;
                break;
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
        let logs = vec![
            vec![20190101,0,1], vec![20190104,3,4],
            vec![20190107,2,3], vec![20190211,1,5],
            vec![20190224,2,4], vec![20190301,0,3],
            vec![20190312,1,2], vec![20190322,4,5]];
        let n = 6;
        let result = Solution::earliest_acq(logs, n);
        assert_eq!(result, 20190301);
    }

    #[test]
    fn example_2() {
        let logs = vec![vec![0,2,0], vec![1,0,1], vec![3,0,3], vec![4,1,2], vec![7,3,1]];
        let n = 4;
        let result = Solution::earliest_acq(logs, n);
        assert_eq!(result, 3);
    }

    #[test]
    fn not_friends() {
        let logs = vec![vec![0,1,2], vec![1,2,3], vec![2,3,4]];
        let n = 5;
        let result = Solution::earliest_acq(logs, n);
        assert_eq!(result, -1);
    }

    #[test]
    fn unordered_timestamps() {
        let logs = vec![vec![9,3,0], vec![0,2,1], vec![8,0,1], vec![1,3,2], vec![2,2,0], vec![3,3,1]];
        let n = 4;
        let result = Solution::earliest_acq(logs, n);
        assert_eq!(result, 2);
    }

}
