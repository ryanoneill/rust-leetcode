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
        let n = nums.len();
        let mut white_begin = 0; // p0
        let mut blue_begin = n - 1; // p2
        let mut i = 0;

        while i <= blue_begin {
            let color = nums[i];
            if color == 0 {
                nums.swap(i, white_begin);
                i += 1;
                white_begin += 1;
            } else if color == 2 {
                nums.swap(i, blue_begin);
                if blue_begin != 0 {
                    blue_begin -= 1;
                } else if i == 0 {
                    i += 1;
                }
            } else {
                i += 1;
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
