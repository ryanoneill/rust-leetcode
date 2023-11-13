use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given a binary tree, find its minimum depth.
///
/// The minimum depth is the number of nodes along the shortest path from the
/// root node down to the nearest leaf node.
///
/// Note: A leaf is a node with no children.
struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.min_depth() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::min_depth(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let root = tree!("[2,null,3,null,4,null,5,null,6]");
        let result = Solution::min_depth(root);
        assert_eq!(result, 5);
    }

    #[test]
    fn root_only() {
        let root = tree!("[10]");
        let result = Solution::min_depth(root);
        assert_eq!(result, 1);
    }

}
