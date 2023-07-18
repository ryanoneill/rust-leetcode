use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Item {
    time: i32,
    target: i32,
}

impl Item {

    fn new(time: i32, target: i32) -> Self {
        Self { time, target }
    }

}

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

    fn to_adjacency_map(times: Vec<Vec<i32>>) -> HashMap<i32, HashMap<i32, i32>> {
        let mut result = HashMap::new();
        for time in times {
            let source = time[0];
            let target = time[1];
            let how_long = time[2];
            result
                .entry(source)
                .or_insert(HashMap::new())
                .insert(target, how_long);
        }
        result
    }

    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let times = Self::to_adjacency_map(times);
        let mut shortest: HashMap<i32, i32> = HashMap::new();

        let mut reachable = BinaryHeap::new();
        let item = Item::new(0, k);
        reachable.push(Reverse(item));

        while !reachable.is_empty() {
            let next_item = reachable.pop().unwrap().0;
            if !shortest.contains_key(&next_item.target) {
                shortest.insert(next_item.target, next_item.time);

                if times.contains_key(&next_item.target) {
                    for (target, time) in &times[&next_item.target] {
                        if !shortest.contains_key(&target) {
                            let total = time + next_item.time;
                            let item = Item::new(total, *target);
                            reachable.push(Reverse(item));
                        }
                    }
                }
            }
        }

        if shortest.len() == (n as usize) {
            shortest.values().max().copied().unwrap_or_default()
        } else {
            -1
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let times = vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]];
        let n = 4;
        let k = 2;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let times = vec![vec![1,2,1]];
        let n = 2;
        let k = 1;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let times = vec![vec![1,2,1]];
        let n = 2;
        let k = 2;
        let result = Solution::network_delay_time(times, n, k);
        assert_eq!(result, -1);
    }

}
