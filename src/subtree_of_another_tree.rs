use crate::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

/// Given the roots of two binary trees `root` and `subRoot`, return `true` if there is a subtree
/// of `root` with the same structure and node values of `subRoot` and `false` otherwise.
///
/// A subtree of a binary tree `tree` is a tree that consists of a node in `tree` and all of this
/// node's descendants. The tree `tree` could also be considered as a subtree of itself.
struct Solution;

impl Solution {

    fn matches(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root, sub_root) {
            (Some(rc_root), Some(rc_sub_root)) => {
                let node_root = rc_root.borrow();
                let node_sub_root = rc_sub_root.borrow();

                let mut result = node_root.val == node_sub_root.val;
                if result {
                    result = Self::matches(&node_root.left, &node_sub_root.left);
                }
                if result {
                    result = Self::matches(&node_root.right, &node_sub_root.right);
                }
                result
            }
            (None, None) => {
                true
            }
            (_, _) => {
                false
            }
        }
    }

    fn is_subtree_worker(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = Self::matches(root, sub_root);
        if !result {
            match root {
                Some(rc) => {
                    let node = rc.borrow();
                    result = Self::is_subtree_worker(&node.left, sub_root);
                    if !result {
                        result = Self::is_subtree_worker(&node.right, sub_root);
                    }
                }
                None => {
                    // do nothing
                }
            }
        }
        result
    }

    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_subtree_worker(&root, &sub_root)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,4,5,1,2]");
        let sub_root = tree!("[4,1,2]");
        let result = Solution::is_subtree(root, sub_root);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let root = tree!("[3,4,5,1,2,null,null,null,null,0]");
        let sub_root = tree!("[4,1,2]");
        let result = Solution::is_subtree(root, sub_root);
        assert!(!result);
    }

}
