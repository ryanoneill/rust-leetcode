use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the sum of all left leaves.
///
/// A leaf is a node with no children. A left leaf is a leaf that is the left child of another
/// node.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut result = 0;
                if node.right.is_some() {
                    result += Self::worker(&node.right);
                }
                if node.left.is_some() {
                    if node.left.is_leaf() {
                        result += node.left.get_value_or_zero();
                    } else {
                        result += Self::worker(&node.left);
                    }
                }
                result
            }
            None => 0
        }

    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::worker(&root)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::sum_of_left_leaves(root);
        assert_eq!(result, 24);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::sum_of_left_leaves(root);
        assert_eq!(result, 0);
    }


}
