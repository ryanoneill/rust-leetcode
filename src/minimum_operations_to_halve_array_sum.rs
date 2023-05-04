use std::collections::BinaryHeap;

/// You are given an array `nums` of positive integers. In one operation, you
/// can choose any number from `nums` and reduce it to exactly half the number.
/// (Note that you may choose this reduced number in future operations.)
///
/// Return the minimum number of operations to reduce the sum of `nums` by at
/// least half.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Wrapped(f64);

impl Eq for Wrapped {}

impl Ord for Wrapped {
    // Based on the constraints of the problem, e.g. the numbers being given
    // as i32s, and the one place where we divide by 2.0, no values should be
    // encountered that cause this Ord definition to be problematic.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

struct Solution;

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum: f64 = 0.0;
        let mut heap = BinaryHeap::new();

        for num in nums {
            let num = num as f64;
            sum += num;
            heap.push(Wrapped(num));
        }

        let half = sum / 2.0;
        while sum > half {
            result += 1;
            let mut value: f64 = heap.pop().unwrap().0;
            sum -= value;
            value /= 2.0;
            sum += value;
            heap.push(Wrapped(value));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![5, 19, 8, 1];
        let result = Solution::halve_array(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 8, 20];
        let result = Solution::halve_array(nums);
        assert_eq!(result, 3);
    }
}
