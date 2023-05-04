use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// You have some number of sticks with positive integer lengths. These lengths
/// are given as an array `sticks`, where `sticks[i]` is the length of the
/// `ith` stick.
///
/// You can connect any two sticks of lengths `x` and `y` into one stick by
/// paying a cost of `x + y`. You must connect all the sticks until there is
/// only one stick remaining.
///
/// Return the minimum cost of connecting all the given sticks into one stick
/// in this way.
struct Solution;

impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut result = 0;

        if sticks.len() > 1 {
            let mut min_heap = BinaryHeap::new();
            for stick in sticks {
                min_heap.push(Reverse(stick));
            }

            while min_heap.len() > 1 {
                let first = min_heap.pop().unwrap().0;
                let second = min_heap.pop().unwrap().0;
                let cost = first + second;
                result += cost;
                min_heap.push(Reverse(cost));
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
        let sticks = vec![2, 4, 3];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 14);
    }

    #[test]
    fn example_2() {
        let sticks = vec![1, 8, 3, 5];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 30);
    }

    #[test]
    fn example_3() {
        let sticks = vec![5];
        let result = Solution::connect_sticks(sticks);
        assert_eq!(result, 0);
    }
}
