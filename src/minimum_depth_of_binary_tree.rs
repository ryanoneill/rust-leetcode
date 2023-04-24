use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;

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
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::Solution;

    #[test]
    fn example_1() {
        // let items = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let seven = TreeNodeAdditions::new(7);
        let fifteen = TreeNodeAdditions::new(15);
        let twenty = TreeNodeAdditions::with_children(20, fifteen, seven);
        let nine = TreeNodeAdditions::new(9);
        let three = TreeNodeAdditions::with_children(3, nine, twenty);
        let result = Solution::min_depth(three);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        // let items = vec![Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)];
        let six = TreeNodeAdditions::new(6);
        let five = TreeNodeAdditions::with_children(5, None, six);
        let four = TreeNodeAdditions::with_children(4, None, five);
        let three = TreeNodeAdditions::with_children(3, None, four);
        let two = TreeNodeAdditions::with_children(2, None, three);
        let result = Solution::min_depth(two);
        assert_eq!(result, 5);
    }

    #[test]
    fn root_only() {
        let root = TreeNodeAdditions::new(10);
        let result = Solution::min_depth(root);
        assert_eq!(result, 1);
    }

}
