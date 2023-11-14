use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

/// Given the `root` of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path
/// from the root node down to the farthest leaf node.
struct Solution;

impl Solution {

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.max_depth() as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::max_depth(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,null,2]");
        let result = Solution::max_depth(root);
        assert_eq!(result, 2);
    }

}
