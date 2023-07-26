struct Solution;

impl Solution {

    fn find_minimum_index(nums: &Vec<i32>, start: usize, end: usize) -> usize {
        let n = nums.len();
        let mid = start + (end - start) / 2;
        let mid_value = nums[mid];
        let next_mid = (mid + 1) % n;
        let next_mid_value = nums[next_mid];
        if mid_value > next_mid_value {
            next_mid
        } else if mid_value >= nums[start] {
            Self::find_minimum_index(nums, mid + 1, end)
        } else {
            Self::find_minimum_index(nums, start, mid - 1)
        }
    }

    fn worker(nums: &Vec<i32>, min_index: usize, target: i32, start: usize, end: usize) -> i32 {
        let n = nums.len();

        let mid = start + (end - start) / 2;
        let adjusted = (mid + min_index) % n;

        let mid_value = nums[adjusted];
        if mid_value == target {
            adjusted as i32
        } else if mid_value < target && mid != end {
            Self::worker(nums, min_index, target, mid + 1, end)
        } else if mid_value > target && mid != start {
            Self::worker(nums, min_index, target, start, mid - 1)
        } else {
            -1
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let result;
        let n = nums.len();
        match n {
            0 => result = -1,
            1 => {
                if nums[0] == target {
                    result = 0;
                } else {
                    result = -1;
                }
            }
            _ => {
                let min_index = Self::find_minimum_index(&nums, 0, n-1);
                result = Self::worker(&nums, min_index, target, 0, n-1);
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
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

}


