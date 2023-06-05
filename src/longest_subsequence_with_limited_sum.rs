/// You are given an integer array `nums` of length `n`, and an integer array `queries` of length
/// `m`.
///
/// Return an array `answer` of length `m` where `answer[i]` is the maximum size of a subsequence
/// that you can take from `nums` such that the sum of its elements is less than or equal to
/// `queries[i]`. 
///
/// A subsequence is an array that can be derived from another array by deleting some or no
/// elements without chaning the order of the remaining elements.
struct Solution;

impl Solution {

    fn to_sums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        let n = nums.len();
        let mut sum = 0;
        for i in 0..n {
            let num = nums[i];
            sum += num;
            nums[i] = sum;
        }

        nums
    }

    fn search(sums: &Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = sums.len();

        while start < end {
            let mid = start + (end - start) / 2;
            if sums[mid] > target {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        start as i32
    }

    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let sums = Self::to_sums(nums);
        let n = queries.len();
        let mut result = vec![0; n];

        for i in 0..n {
            let query = queries[i];
            let count = Self::search(&sums, query);
            result[i] = count;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![4,5,2,1];
        let queries = vec![3,10,21];
        let result = Solution::answer_queries(nums, queries);
        assert_eq!(result, vec![2,3,4]);
    }

    #[test]
    fn example_2() {
        let nums = vec![2,3,4,5];
        let queries = vec![1];
        let result = Solution::answer_queries(nums, queries);
        assert_eq!(result, vec![0]);
    }

}
