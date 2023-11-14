use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the length of the diameter of the
/// tree.
///
/// The diameter of a binary tree is the length of the longest path between any
/// two nodes in a tree. This pay may or may not pass through the `root`.
///
/// The length of a path between two nodes is represented by the number of
/// edges between them.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();

                let left_depth = node.left.max_depth() as i32;
                let right_depth = node.right.max_depth() as i32;
                let edges_between = left_depth + right_depth;

                let left_diameter = Self::worker(&node.left);
                let right_diameter = Self::worker(&node.right);

                edges_between.max(left_diameter).max(right_diameter)
            }
            None => 0,
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::worker(&root)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,4,5]");
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2]");
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 1);
    }

}
