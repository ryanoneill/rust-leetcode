/// A permutation of an array of integers is an arrangement of its members into a sequence of
/// linear order.
///
/// * For example, for `arr = [1,2,3]`, the following are all permutations of `arr`: `[1,2,3],
/// [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]`
///
/// The next permutation of an array of integers is the next lexicographically greater permutation
/// of its integer. More formally, if all the permutations of the array are sorted in one container
/// according to their lexicographical order, then the next permutation of that array is the
/// permutation that follow it in the sorted container. If such arrangement is not possible, the
/// array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
///
/// * For example, the next permutation of `arr = [1,2,3]` is `[1,3,2]`.
/// * Similarly, the next permutation of `arr = [2,3,1]` is `[3,1,2]`.
/// * While the next permutation of `arr = [3,2,1]` is `[1,2,3]` because `[3,2,1]` does not have a
///   lexicographical larger rearrangement.
///
/// Given an array of integers `nums`, find the next permutation of `nums`.
///
/// The replacement must be in place and use only constant extra memory.
struct Solution;

impl Solution {

    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n > 1 {
            let mut i = n - 2;
            let mut all = false;
            while nums[i+1] <= nums[i] {
                if i == 0 {
                    all = true;
                    break;
                } else {
                    i -= 1;
                }
            }
            if all {
                Self::reverse(nums, 0);
            } else {
                let mut j = n - 1;
                while nums[j] <= nums[i] {
                    j -= 1;
                }
                nums.swap(i, j);
                Self::reverse(nums, i+1);
            }
        }
    }

    fn reverse(nums: &mut Vec<i32>, start: usize) {
        let mut i = start;
        let mut j = nums.len() - 1;
        while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1,2,3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![3,2,1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,2,3]);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1,1,5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,5,1]);
    }

}
