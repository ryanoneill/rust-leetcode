use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree and an integer `targetSum`, return `true`
/// if the tree has a root-to-leaf path such that adding up all the values
/// along the path equals `targetSum`.
///
/// A leaf is a node with no children.
struct Solution;

impl Solution {
    fn has_path_sum_within(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match node {
            Some(rc) => {
                let node = rc.borrow();
                let new_target = target_sum - node.val;

                if node.left.is_none() && node.right.is_none() {
                    new_target == 0
                } else {
                    let has_on_left = Self::has_path_sum_within(&node.left, new_target);
                    let has_on_right = Self::has_path_sum_within(&node.right, new_target);

                    has_on_left || has_on_right
                }
            }
            None => false,
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_within(&root, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[5,4,8,11,null,13,4,7,2,null,null,null,1]");
        let result = Solution::has_path_sum(root, 22);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2,3]");
        let result = Solution::has_path_sum(root, 5);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let root = tree!("[]");
        let result = Solution::has_path_sum(root, 0);
        assert!(!result);
    }

    #[test]
    fn must_be_leaf() {
        let root = tree!("[1,2]");
        let result = Solution::has_path_sum(root, 1);
        assert!(!result);
    }

    #[test]
    fn negative_target_sum() {
        let root = tree!("[1,-2,-3,1,3,-2,null,-1]");
        let result = Solution::has_path_sum(root, -1);
        assert!(result);
    }
}
