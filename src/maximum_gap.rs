#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bucket {
    in_use: bool,
    minimum: i32,
    maximum: i32,
}

impl Bucket {

    fn new() -> Self {
        Self { in_use: false, minimum: i32::MAX, maximum: i32::MIN }
    }

    fn add_value(&mut self, value: i32) {
        self.in_use = true;
        self.minimum = self.minimum.min(value);
        self.maximum = self.maximum.max(value);
    }

}

/// Given an integer array `nums`, return the maximum difference between two
/// successive elements in its sorted form. If the array contains less than two
/// elements, return `0`.
///
/// You must write an algorithm that runs in linear time and uses linear space.
struct Solution;

impl Solution {

    fn find_min_max(nums: &Vec<i32>) -> (i32, i32) {
        let mut min_result = i32::MAX;
        let mut max_result = i32::MIN;

        for &num in nums {
            min_result = min_result.min(num);
            max_result = max_result.max(num);
        }

        (min_result, max_result)
    }

    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = nums.len();
        if n >= 2 {
            let (min_value, max_value) = Self::find_min_max(&nums);
            let diff = max_value - min_value;

            let bucket_size = (diff / (n as i32 - 1)).max(1);
            let bucket_count = ((diff / bucket_size) + 1) as usize;
            let mut buckets = vec![Bucket::new(); bucket_count];

            for num in nums {
                let bucket_i = ((num - min_value) / bucket_size) as usize;
                buckets[bucket_i].add_value(num);
            }

            let mut previous_bucket_max = min_value;
            for bucket in buckets {
                if bucket.in_use {
                    result = result.max(bucket.minimum - previous_bucket_max);
                    previous_bucket_max = bucket.maximum;
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
        let nums = vec![3,6,9,1];
        let result = Solution::maximum_gap(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![10];
        let result = Solution::maximum_gap(nums);
        assert_eq!(result, 0);
    }

}
