use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// You are given the `root` of a binary search tree (BST) and an integer
/// `val`.
///
/// Find the node in the BST that the node's value equals `val` and return the
/// subtree rooted with that node. If such a node does not exist, return
/// `null`.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val;
                if val == value {
                    root.clone()
                } else if val < value && node.left.is_some() {
                    Self::worker(&node.left, val)
                } else if val > value && node.right.is_some() {
                    Self::worker(&node.right, val)
                } else {
                    None
                }
            }
            None => {
                None
            }
        }
    }

    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::worker(&root, val)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[4,2,7,1,3]");
        let val = 2;
        let result = Solution::search_bst(root, val);
        assert_tree!(result, "[2,1,3]");
    }

    #[test]
    fn example_2() {
        let root = tree!("[4,2,7,1,3]");
        let val = 5;
        let result = Solution::search_bst(root, val);
        assert_tree!(result, "[]");
    }

}
