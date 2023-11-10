use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given an integer array `nums` where the elements are sorted in ascending order, convert it to a
/// height-balanced binary search tree.
struct Solution;

impl Solution {

    fn worker(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result: Option<Rc<RefCell<TreeNode>>> = None;
        if left <= right {
            let mid = left + (right - left) / 2;
            let val = nums[mid];

            result = TreeNodeAdditions::new(val);
            if mid > 0 {
                let left_child = Self::worker(nums, left, mid - 1);
                result.set_left(left_child);
            }
            let right_child = Self::worker(nums, mid + 1, right);
            result.set_right(right_child);
        }
        result
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = nums.len();
        Self::worker(&nums, 0, n-1)
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let result = Solution::sorted_array_to_bst(nums);
        let codec = Codec::new();
        assert_eq!(codec.serialize(result), "[0,-10,5,null,-3,null,9]");
    }

    #[test]
    fn example_2() {
        let nums = vec![1,3];
        let result = Solution::sorted_array_to_bst(nums);
        let codec = Codec::new();
        assert_eq!(codec.serialize(result), "[1,null,3]");
    }

}
