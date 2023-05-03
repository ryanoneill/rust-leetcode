/// Given an integer array `nums`, rotate the array to the right by `k` steps,
/// where `k` is non-negative.
struct Solution;

impl Solution {

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;
        let (n1, n2) = nums.split_at(n - k);
        *nums = n2.iter().chain(n1.iter()).copied().collect();
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![-1,-100,3,99];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![3,99,-1,-100]);
    }

}
