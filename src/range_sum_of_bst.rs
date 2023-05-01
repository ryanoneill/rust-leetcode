use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` node of a binary search tree and two integers `low` and
/// `high`, return the sum of values of all nodes with a value in the
/// inclusive range `[low, high]`.
struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(rc) => {
                let mut result = 0;

                let node = rc.borrow();
                let value = node.val;

                if low <= value && value <= high {
                    result += value;
                }
                if low < value && node.left.is_some() {
                    result += Self::range_sum_bst(node.left.clone(), low, high);
                }
                if value < high && node.right.is_some() {
                    result += Self::range_sum_bst(node.right.clone(), low, high);
                }

                result
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;

    #[test]
    fn example_1() {
        let data = "[10,5,15,3,7,null,18]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::range_sum_bst(root, 7, 15);
        assert_eq!(result, 32);
    }

    #[test]
    fn example_2() {
        let data = "[10,5,15,3,7,13,18,1,null,6]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::range_sum_bst(root, 6, 10);
        assert_eq!(result, 23);
    }
}
