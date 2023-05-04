use std::collections::BinaryHeap;

/// You are given a 0-indexed integer array `piles`, where `piles[i]`
/// represents the number of stones in the `ith` pile, and an integer `k`.
/// You should apply the following operation exactly `k` times:
///
/// * Choose any `piles[i]` and remove `floor(piles[i] / 2)` stones from it.
///
/// Notice that you can apply the operation on the same pile more than once.
///
/// Return the minimum possible total of stones remaining after applying the
/// `k` operations.
///
/// `floor(x)` is the greatest integer that is smaller than or equal to `x`
/// (i.e., rounds `x` down).
struct Solution;

impl Solution {
    // Sum should be under 2 billion
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut max_heap = BinaryHeap::new();
        for pile in piles {
            sum += pile;
            max_heap.push(pile);
        }
        for _ in 0..k as usize {
            let mut num = max_heap.pop().unwrap();
            let removed = num / 2;
            sum -= removed;
            num = num - removed;
            max_heap.push(num);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let piles = vec![5, 4, 9];
        let k = 2;
        let result = Solution::min_stone_sum(piles, k);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_2() {
        let piles = vec![4, 3, 6, 7];
        let k = 3;
        let result = Solution::min_stone_sum(piles, k);
        assert_eq!(result, 12);
    }
}
