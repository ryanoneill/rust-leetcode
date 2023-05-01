use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
use std::rc::Rc;

/// Given the `root` of a binary tree, find the maximum value `v` for which
/// there exist different nodes `a` and `b` where `v = |a.val - b.val|` and
/// `a` is an ancestor of `b`.
///
/// A node `a` is an ancestor of `b` if either: any child of `a` is equal
/// to `b` or any child of `a` is an ancestor of `b`.
struct Solution;

impl Solution {
    fn max_ancestor_diff_high_low(
        root: Option<Rc<RefCell<TreeNode>>>,
        highest: i32,
        lowest: i32,
    ) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val;
                let new_highest = max(value, highest);
                let new_lowest = min(value, lowest);
                let diff = new_highest - new_lowest;

                let left =
                    Self::max_ancestor_diff_high_low(node.left.clone(), new_highest, new_lowest);
                let right =
                    Self::max_ancestor_diff_high_low(node.right.clone(), new_highest, new_lowest);

                max(max(diff, left), right)
            }
            None => 0,
        }
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            Self::max_ancestor_diff_high_low(root, i32::min_value(), i32::max_value())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;

    #[test]
    fn example_1() {
        let data = "[8,3,10,1,6,null,14,null,null,4,7,13]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let data = "[1,null,2,null,0,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(result, 3);
    }
}
