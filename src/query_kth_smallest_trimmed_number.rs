use std::cmp::Ordering;

struct TrimmedNumber {
    digits: Vec<char>,
    index: usize,
    trim: usize
}

impl TrimmedNumber {

    pub fn new(num: &str, index: usize) -> Self {
        let digits: Vec<char> = num.chars().collect();
        let trim = digits.len();
        Self { digits, index, trim }
    }

    pub fn set_trim(&mut self, trim: usize) {
        if trim <= self.digits.len() {
            self.trim = trim;
        }
    }

}

impl PartialEq for TrimmedNumber {

    fn eq(&self, other: &Self) -> bool {
        if self.index == other.index {
            let left = self.digits.len() - self.trim;
            self.digits[left..].eq(&other.digits[left..])
        } else { false }
    }

}

impl Eq for TrimmedNumber { }

impl PartialOrd for TrimmedNumber {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.digits.len() - self.trim;
        match self.digits[left..].partial_cmp(&other.digits[left..]) {
            Some(Ordering::Equal) => {
                self.index.partial_cmp(&other.index)
            }
            result => result
        }
    }

}

impl Ord for TrimmedNumber {

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
    
}

/// You are given a 0-indexed array of strings `nums`, where each string is of
/// equal length and consists of only digits.
///
/// You are also given a 0-indexed 2D integer array `queries` where
/// `queries[i] = [ki, trimi]`. For each `queries[i]`, you need to:
///
/// * Trim each number in `nums` to its rightmost `trimi` digits.
///
/// * Determine the index of the `kith` smallest trimmed number in `nums`.
///   If two trimmed numbers are equal, the number with the lower index
///   is considered to be smaller.
///
/// * Reset each number in `nums` to its original length.
///
/// Return an array `answer` of the same length as `queries`, where `answer[i]`
/// is the answer to the `ith` query.
struct Solution;

impl Solution {

    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut working = Vec::new();
        for i in 0..n {
            let num = &nums[i];
            let trimmed = TrimmedNumber::new(num, i);
            working.push(trimmed);
        }

        let mut result = Vec::new();
        for query in queries {
            let k = query[0] as usize - 1;
            let trim = query[1] as usize;
            working.iter_mut().for_each(|t| { t.set_trim(trim); });
            working.sort();

            result.push(working[k].index as i32);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let orig = vec!["102", "473", "251", "814"];
        let nums = orig.iter().map(|s| s.to_string()).collect();
        let queries = vec![vec![1,1], vec![2,3], vec![4,2], vec![1,2]];
        let result = Solution::smallest_trimmed_numbers(nums, queries);
        assert_eq!(result, vec![2,2,1,0]);
    }

    #[test]
    fn example_2() {
        let orig = vec!["24", "37", "96", "04"];
        let nums = orig.iter().map(|s| s.to_string()).collect();
        let queries = vec![vec![2,1], vec![2,2]];
        let result = Solution::smallest_trimmed_numbers(nums, queries);
        assert_eq!(result, vec![3,0]);
    }

}

