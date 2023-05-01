use std::collections::BinaryHeap;

/// You are given an array of integers `stones` where `stones[i]` is the weight
/// of the `ith` stone.
///
/// We are playing a game with stones. On each turn, we choose the heaviest two
/// stones and smash them together. Supposed the heaviest two stones have
/// weights `x` and `y` with `x <= y`. The result of this smash is:
///
/// * If `x == y`, both stones are destroyed, and
/// * If `x != y`, the stone of weight `x` is destroyed, and the stone of
///   weight `y` has new weight `y - x`.
///
/// At the end of the game, there is at most one stone left.
///
/// Return the weight of the last remaining stone. If there are no stones left,
/// return 0.
struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into();

        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();

            if x != y {
                heap.push(y - x);
            }
        }

        heap.pop().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let stones = vec![1];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 1);
    }

    #[test]
    fn end_with_no_stones() {
        let stones = vec![10, 10];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 0);
    }

    #[test]
    fn end_with_one_stone_with_heavier_weight() {
        let stones = vec![10, 7];
        let result = Solution::last_stone_weight(stones);
        assert_eq!(result, 3);
    }
}
