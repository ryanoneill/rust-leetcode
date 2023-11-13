use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the roots of two binary trees `p` and `q`, write a function to check
/// if they are the same or not.
///
/// Two binary trees are considered the same if they are structurally identical, and
/// the nodes have the same value.
struct Solution;

impl Solution {

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p_rc), Some(q_rc)) => {
                let p_node = p_rc.borrow();
                let q_node = q_rc.borrow();

                let mut result = p_node.val == q_node.val;
                result = result && Self::is_same_tree(p_node.left.clone(), q_node.left.clone());
                result = result && Self::is_same_tree(p_node.right.clone(), q_node.right.clone());

                result
            }
            (None, None) => true,
            _ => false
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let p = tree!("[1,2,3]");
        let q = tree!("[1,2,3]");
        let result = Solution::is_same_tree(p, q);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let p = tree!("[1,2]");
        let q = tree!("[1,null,2]");
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let p = tree!("[1,2,1]");
        let q = tree!("[1,1,2]");
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

}
