use std::collections::HashSet;
use std::collections::VecDeque;

/// Given an array of non-negative integers `arr`, you are initially positioned
/// at `start` index of the array. When you are at index `i`, you can jump to
/// `i + arr[i]` or `i - arr[i]`, check if you can reach to any index with
/// value 0.
///
/// Notice that you can not jump outside of the array at any time.
struct Solution;

impl Solution {

    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let start = start as usize;

        let mut result = false;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        seen.insert(start);
        queue.push_back(start);
        while !queue.is_empty() {
            let index = queue.pop_front().unwrap();
            let away = arr[index] as usize;
            if away == 0 {
                result = true;
                break;
            } else {
                let forward = index + away;
                if forward < n && !seen.contains(&forward) {
                    seen.insert(forward);
                    queue.push_back(forward);
                }
                if index >= away {
                    let backward = index - away;
                    if !seen.contains(&backward) {
                        seen.insert(backward);
                        queue.push_back(backward);
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
        let arr = vec![4,2,3,0,3,1,2];
        let start = 5;
        let result = Solution::can_reach(arr, start);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let arr = vec![4,2,3,0,3,1,2];
        let start = 0;
        let result = Solution::can_reach(arr, start);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let arr = vec![3,0,2,1,2];
        let start = 0;
        let result = Solution::can_reach(arr, start);
        assert!(!result);
    }

}
