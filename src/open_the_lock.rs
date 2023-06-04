use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

/// You have a lock in front of you with 4 circular wheels. Each wheel has 10
/// slots: `'0'`, `'1'`, `'2'`, `'3'`, `'4'`, `'5'`, `'6'`, `'7'`, `'8'`, `'9'.
/// The weels can rotate freely and wrap around: for example we can turn `'9'`
/// to be `'0'`, or `'0' to be `'9'`. Each move consists of turning one wheel
/// one slot.
///
/// The lock initially starts at `0000`, a string representing the state of the
/// 4 wheels.
///
/// You are given a list of `deadends` dead ends, meaning if the lock displays
/// any of these codes, the wheels of the lock will stop turning and you will
/// be unable to open it.
///
/// Given a `target` representing the value of the wheels that will unlock the
/// lock, return the minimum total number of turns required to open the lock,
/// or -1 if it is impossible.
struct Solution;

impl Solution {

    fn one_steps(combo: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(8);
        for i in 0..4 {
            let mut up = combo.clone();
            up[i] = (up[i] + 1) % 10;
            result.push(up);

            let mut down = combo.clone();
            down[i] = (down[i] + 9) % 10;
            result.push(down);
        }

        result
    }

    fn lock_string_to_vec(s: String) -> Vec<i32> {
        s.chars()
            .map(|d| match d {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _   => unreachable!("Invalid Lock Sequence")
            })
            .collect()
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<Vec<i32>> = HashSet::from_iter(deadends.into_iter().map(Self::lock_string_to_vec));
        let target = Self::lock_string_to_vec(target);
        let start = vec![0,0,0,0];

        let mut result = 0;
        let mut finished = false;
        let mut queue = VecDeque::new();
        let mut seen = deadends;

        if !seen.contains(&start) {
            queue.push_back(start.clone());
            seen.insert(start);
        }
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let candidate = queue.pop_front().unwrap();
                if candidate == target {
                    finished = true;
                    break;
                } else {
                    for step in Self::one_steps(candidate) {
                        if !seen.contains(&step) {
                            seen.insert(step.clone());
                            queue.push_back(step);
                        }
                    }
                }
            }
            if finished {
                break;
            } else {
                result += 1;
            }
        }

        if finished { result } else { -1 }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn vec_strings(items: Vec<&str>) -> Vec<String> {
        items
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        let deadends = vec_strings(vec!["0201","0101","0102","1212","2002"]);
        let target = String::from("0202");
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let deadends = vec_strings(vec!["8888"]);
        let target = String::from("0009");
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let deadends = vec_strings(vec!["8887","8889","8878","8898","8788","8988","7888","9888"]);
        let target = String::from("8888");
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn start_deadend() {
        let deadends = vec_strings(vec!["0000"]);
        let target = String::from("8888");
        let result = Solution::open_lock(deadends, target);
        assert_eq!(result, -1);
    }

}
