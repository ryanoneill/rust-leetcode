use std::collections::BTreeMap;

/// The median is the middle value in an ordered integer list. If the size of the list is even,
/// there is no middle value. So the median is the mean of the two middle values.
///
/// * For examples, if `arr = [2,3,4]`, the median is `3`.
///
/// * For examples, if `arr = [1,2,3,4]`, the median is `(2 + 3) / 2 = 2.5`.
///
/// You are given an integer array `nums` and an integer `k`. There is a sliding window of size `k`
/// which is moving from the very left of the array to the very right. You can only see the `k`
/// numbers in the window. Each time the sliding window moves right by one position.
///
/// Return the median array for each window in the original array. Answers within `10^-5` of the
/// actual value will be accepted.
struct Solution;

#[derive(Debug)]
struct MedianFinder {
    above: BTreeMap<i32, usize>,
    below: BTreeMap<i32, usize>,

    above_len: usize,
    below_len: usize,
}

impl MedianFinder {

    pub fn new() -> Self {
        Self {
            above: BTreeMap::new(),
            below: BTreeMap::new(),

            above_len: 0,
            below_len: 0,
        }
    }

    fn max_below(&self) -> i32 {
        // Last Key Value is not available until 1.66 which was released in December 2022.
        // LeetCode only supports an older version of the compiler unfortunately.
        //
        // if let Some((key, _)) = self.below.last_key_value() {
        //     *key
        // } else {
        //     0
        // }
        *self.below.keys().rev().next().unwrap()
    }

    fn min_above(&self) -> i32 {
        // First Key Value is not available until 1.66 which was released in December 2022.
        // LeetCode only supports an older version of the compiler unfortunately.
        //
        // if let Some((key, _)) = self.above.first_key_value() {
        //     *key
        // } else {
        //     0
        // }
        *self.above.keys().next().unwrap()
    }

    fn add_above(&mut self, num: i32) {
        self.above
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.above_len += 1;
    }

    fn add_below(&mut self, num: i32) {
        self.below
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.below_len += 1;
    }

    fn remove_above(&mut self, num: i32) {
        if self.above.contains_key(&num) {
            let mut value = self.above[&num];
            if value == 1 {
                self.above.remove(&num);
            } else {
                value -= 1;
                self.above.insert(num, value);
            }
            self.above_len -= 1;
        }
    }

    fn remove_below(&mut self, num: i32) {
        if self.below.contains_key(&num) {
            let mut value = self.below[&num];
            if value == 1 {
                self.below.remove(&num);
            } else {
                value -= 1;
                self.below.insert(num, value);
            }
            self.below_len -= 1;
        }
    }

    pub fn add(&mut self, num: i32) {
        self.add_below(num);
        let max = self.max_below();
        self.remove_below(max);
        self.add_above(max);

        if self.above_len > self.below_len {
            let min = self.min_above();
            self.remove_above(min);
            self.add_below(min);
        }
    }

    pub fn remove(&mut self, num: i32) {
        if self.below_len > self.above_len {
            if self.below.contains_key(&num) {
                self.remove_below(num);
            } else {
                self.remove_above(num);
                let max = self.max_below();
                self.remove_below(max);
                self.add_above(max);
            }
        } else if self.below.contains_key(&num) {
            self.remove_below(num);
            let min = self.min_above();
            self.remove_above(min);
            self.add_below(min);
        } else {
            self.remove_above(num);
        }
    }

    pub fn median(&self) -> f64 {
        if self.above_len == self.below_len {
            let max = self.max_below() as f64;
            let min = self.min_above() as f64;
            (max + min) / 2.0
        } else {
            self.max_below() as f64
        }
    }

}

impl Solution {

    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();

        let mut results = Vec::new();
        let mut median_finder = MedianFinder::new();

        for i in 0..k {
            let num = nums[i];
            median_finder.add(num);
        }
        results.push(median_finder.median());

        for i in k..n {
            let old = nums[i-k];
            median_finder.remove(old);
            let num = nums[i];
            median_finder.add(num);
            results.push(median_finder.median());
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let k = 3;
        let result = Solution::median_sliding_window(nums, k);
        assert_eq!(result, vec![1.0,-1.0,-1.0,3.0,5.0,6.0]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1,2,3,4,2,3,1,4,2];
        let k = 3;
        let result = Solution::median_sliding_window(nums, k);
        assert_eq!(result, vec![2.0,3.0,3.0,3.0,2.0,3.0,2.0]);
    }

    #[test]
    fn example_3() {
        let nums = vec![i32::MAX, i32::MAX];
        let k = 2;
        let result = Solution::median_sliding_window(nums, k);
        assert_eq!(result, vec![i32::MAX as f64]);
    }

}


