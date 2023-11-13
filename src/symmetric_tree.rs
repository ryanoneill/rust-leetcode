use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, check whether it is a mirror of itself (i.e., symmetric
/// around its center).
struct Solution;

impl Solution {

    fn worker(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (Some(rc1), Some(rc2)) => {
                let node1 = rc1.borrow();
                let node2 = rc2.borrow();

                let mut result = node1.val == node2.val;
                result = result && Self::worker(&node1.left, &node2.right);
                result = result && Self::worker(&node1.right, &node2.left);

                result
            }
            (Some(_rc1), None) => {
                false
            }
            (None, Some(_rc2)) => {
                false
            }
            (None, None) => {
                true
            }
        }

    }
    
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                Self::worker(&node.left, &node.right)
            }
            None => {
                false
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,2,3,4,4,3]");
        let result = Solution::is_symmetric(root);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2,2,null,3,null,3]");
        let result = Solution::is_symmetric(root);
        assert!(!result);
    }

}
