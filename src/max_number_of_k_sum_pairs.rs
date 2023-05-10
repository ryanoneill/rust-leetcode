use std::collections::HashMap;
use std::collections::HashSet;

struct Indices {
    items: HashMap<i32, HashSet<usize>>,
}

impl Indices {

    fn new() -> Self {
        Self { items: HashMap::new() }
    }

    fn add(&mut self, num: i32, i: usize) {
        self.items
            .entry(num)
            .or_insert(HashSet::new())
            .insert(i);
    }

    fn contains(&self, num: i32, i: usize) -> bool {
        self.items.get(&num).map(|s| s.contains(&i)).unwrap_or_default()
    }

    fn get_index(&self, num: i32) -> Option<usize> {
        self.items.get(&num).and_then(|s| s.iter().next()).copied()
    }

    fn remove(&mut self, num: i32, i: usize) {
        self.items
            .entry(num)
            .and_modify(|s| { s.remove(&i); });
    }

}

/// You are given an integer array `nums` and an integer `k`.
///
/// In one operation, you can pick two numbers from the array whose sum equals
/// `k` and remove them from the array.
///
/// Return the maximum number of operations you can perform on the array.
struct Solution;

impl Solution {

    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut indices = Indices::new();

        for i in 0..n {
            let num = nums[i];
            indices.add(num, i);
        }

        let mut result = 0;
        for i in 0..n {
            let num = nums[i];
            if indices.contains(num, i) {
                indices.remove(num, i);

                let diff = k - num;
                match indices.get_index(diff) {
                    Some(i) => {
                        result += 1;
                        indices.remove(diff, i);
                    }
                    None => { }
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
        let nums = vec![1,2,3,4];
        let k = 5;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![3,1,3,4,3];
        let k = 6;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, 1);
    }

}
