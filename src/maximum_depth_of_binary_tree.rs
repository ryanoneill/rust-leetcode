use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

/// Given the `root` of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path
/// from the root node down to the farthest leaf node.
struct Solution;

impl Solution {

    fn get_max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(rc) => {
                let node = rc.borrow();
                let left = Self::get_max_depth(&node.left);
                let right = Self::get_max_depth(&node.right);

                1 + max(left, right)
            }
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_max_depth(&root)
    }

}
