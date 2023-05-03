use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
        let sticks = vec![1,8,3,5];
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
