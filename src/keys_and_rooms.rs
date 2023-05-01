use std::collections::HashSet;
use std::collections::VecDeque;

/// There are `n` rooms labeled from `0` to `n-1` and all the rooms are locked
/// except for room `0`. Your goal is to visit all the rooms. However, you
/// cannot enter a locked room without having its key.
///
/// When you visit a room, you may find a set of distinct keys in it. Each key
/// has a number on it, denoting which room it unlocks, and you can take all of
/// them with you to unlock the other rooms.
///
/// Given an array `rooms` where `rooms[i]` is the set of keys that you can
/// obtain if you visited room `i`, return `true` if you can visit all the
/// rooms, or `false` otherwise.
struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(0);
        while !queue.is_empty() {
            let room = queue.pop_front().unwrap();
            seen.insert(room);

            let keys = &rooms[room];
            for key in keys {
                let key = *key as usize;
                if !seen.contains(&key) {
                    queue.push_back(key);
                }
            }
        }

        seen.len() == n
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        let result = Solution::can_visit_all_rooms(rooms);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        let result = Solution::can_visit_all_rooms(rooms);
        assert!(!result);
    }
}
