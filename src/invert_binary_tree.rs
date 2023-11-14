use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, invert the tree, and return its root.
struct Solution;

impl Solution {

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        if root.is_some() {
            let mut left = root.take_left();
            let mut right = root.take_right();

            left = Self::invert_tree(left);
            right = Self::invert_tree(right);

            root.set_left(right);
            root.set_right(left);
        }
        root
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[4,2,7,1,3,6,9]");
        let result = Solution::invert_tree(root);
        assert_tree!(result, "[4,7,2,9,6,3,1]");
    }

    #[test]
    fn example_2() {
        let root = tree!("[2,1,3]");
        let result = Solution::invert_tree(root);
        assert_tree!(result, "[2,3,1]");
    }

    #[test]
    fn example_3() {
        let root = tree!("[]");
        let result = Solution::invert_tree(root);
        assert_tree!(result, "[]");
    }

}
