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
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;
    use super::Solution;

    // TODO: Remove creation of individual nodes when from_vec is working.

    #[test]
    fn example_1() {
        // let items = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let nine = TreeNodeAdditions::new(9);
        let fifteen = TreeNodeAdditions::new(15);
        let seven = TreeNodeAdditions::new(7);
        let twenty = TreeNodeAdditions::with_children(20, fifteen, seven);
        let three = TreeNodeAdditions::with_children(3, nine, twenty);
        let result = Solution::max_depth(three);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        // let items = vec![Some(1), None, Some(2)];
        let two = TreeNodeAdditions::new(2);
        let one = TreeNodeAdditions::with_children(1, None, two);
        let result = Solution::max_depth(one);
        assert_eq!(result, 2);
    }

}
