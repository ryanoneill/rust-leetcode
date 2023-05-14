/// Given an array `nums` with `n` objects colored red, white, or blue,
/// sort them in-place so that objects of the same color are adjacent, with
/// the colors in the order red, white, and blue.
///
/// We will use the integers `0`, `1`, and `2` to represent the color red
/// white, and blue, respectively.
///
/// You must solve this problem without using the library's sort function.
struct Solution;

impl Solution {

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red_count = 0;
        let mut white_count = 0;

        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            if num == 0 {
                red_count += 1;
            } else if num == 1 {
                white_count += 1;
            } 
        }

        for i in 0..n {
            if i < red_count {
                nums[i] = 0;
            } else if i < white_count + red_count {
                nums[i] = 1;
            } else {
                nums[i] = 2;
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![2,0,1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }

    #[test]
    fn single_blue() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }

    #[test]
    fn all_blue() {
        let mut nums = vec![2,2,2,2,2,2,2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2,2,2,2,2,2,2]);
    }

}
