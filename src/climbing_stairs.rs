use std::collections::HashMap;

/// You are climbing a staircase. It takes `n` steps to reach the top.
///
/// Each time you can either climb `1` or `2` steps. In how many distinct ways
/// can you climb to the top?
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ways = HashMap::new();
        Self::worker(n, &mut ways)
    }

    fn worker(n: i32, ways: &mut HashMap<i32, i32>) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => {
                if ways.contains_key(&n) {
                    ways[&n]
                } else {
                    let result = Self::worker(n - 1, ways) + Self::worker(n - 2, ways);
                    ways.insert(n, result);
                    result
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 2;
        let result = Solution::climb_stairs(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let n = 3;
        let result = Solution::climb_stairs(n);
        assert_eq!(result, 3);
    }
}
