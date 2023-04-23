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
            None => { false }
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_within(&root, target_sum)
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
        // let items = vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)];
        let one = TreeNodeAdditions::new(1);
        let four_right = TreeNodeAdditions::with_children(4, None, one);
        let thirteen = TreeNodeAdditions::new(13);
        let eight = TreeNodeAdditions::with_children(8, thirteen, four_right);

        let two = TreeNodeAdditions::new(2);
        let seven = TreeNodeAdditions::new(7);
        let eleven = TreeNodeAdditions::with_children(11, seven, two);
        let four_left = TreeNodeAdditions::with_children(4, eleven, None);

        let five = TreeNodeAdditions::with_children(5, four_left, eight);
        let result = Solution::has_path_sum(five, 22);
        assert!(result);
    }

    #[test]
    fn example_2() {
        // let items = vec![Some(1), Some(2), Some(3)]
        let three = TreeNodeAdditions::new(3);
        let two = TreeNodeAdditions::new(2);
        let one = TreeNodeAdditions::with_children(1, two, three);
        let result = Solution::has_path_sum(one, 5);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        // let items = vec![]
        let result = Solution::has_path_sum(None, 0);
        assert!(!result);
    }

    #[test]
    fn must_be_leaf() {
        // let items = vec![Some(1), Some(2)];
        let two = TreeNodeAdditions::new(2);
        let one = TreeNodeAdditions::with_children(1, two, None);
        let result = Solution::has_path_sum(one, 1);
        assert!(!result);
    }

    #[test]
    fn negative_target_sum() {
        // let items = vec![Some(1), Some(-2), Some(-3), Some(1), Some(3), Some(-2), None, Some(-1)];
        let neg_two_right = TreeNodeAdditions::new(-2);
        let neg_three = TreeNodeAdditions::with_children(-3, neg_two_right, None);

        let three = TreeNodeAdditions::new(3);
        let neg_one = TreeNodeAdditions::new(-1);
        let one_left = TreeNodeAdditions::with_children(1, neg_one, None);
        let neg_two_left = TreeNodeAdditions::with_children(-2, one_left, three);

        let one = TreeNodeAdditions::with_children(1, neg_two_left, neg_three);
        let result = Solution::has_path_sum(one, -1);
        assert!(result);
    }

}
