/// You are given a network of `n` nodes, labeled from `1` to `n`. You are also
/// given `times`, a list of travel times as directed edges
/// `times[i] = (ui, vi, wi)`, where `ui` is the source node, `vi` is the
/// target node, and `wi` is the time it takes for a signal to travel from
/// source to target.
///
/// We will send a signal from a given node `k`. Return the minimum time it
/// takes for all the `n` nodes to receive the signal. If it is impossible for
/// all the `n` nodes to receive the signal, return `-1`.
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn network_delay_time(_times: Vec<Vec<i32>>, _n: i32, _k: i32) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let times = vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]];
        let n = 4;
        let k = 2;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, 2);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let times = vec![vec![1,2,1]];
        let n = 2;
        let k = 1;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, 1);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let times = vec![vec![1,2,1]];
        let n = 2;
        let k = 2;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, -1);
    }

}
