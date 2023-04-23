use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given a binary tree `root`, a node X in the tree is named good if in the
/// path from root to X there are no nodes with a value greater than X.
///
/// Return the number of good nodes in the binary tree.
struct Solution;

impl Solution {

    fn count_good_nodes(root: &Option<Rc<RefCell<TreeNode>>>, max_seen: i32) -> usize {
        match root {
            None => { 0 }
            Some(rc) => {
                let node = rc.borrow();
                let mut result = 0;
                let mut new_max = max_seen;

                if node.val >= max_seen {
                    new_max = node.val;
                    result += 1;
                }

                result += Self::count_good_nodes(&node.left, new_max);
                result += Self::count_good_nodes(&node.right, new_max);

                result
            }
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_good_nodes(&root, i32::min_value()) as i32
    }

}

#[cfg(test)]
mod tests {
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;

    use super::Solution;

    #[test]
    fn example_1() {
        // let items = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
        let five = TreeNodeAdditions::new(5);
        let one_right = TreeNodeAdditions::new(1);
        let four = TreeNodeAdditions::with_children(4, one_right, five);

        let three_left = TreeNodeAdditions::new(3);
        let one_left = TreeNodeAdditions::with_children(1, three_left, None);

        let three = TreeNodeAdditions::with_children(3, one_left, four);
        let result = Solution::good_nodes(three);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        // let items = vec![Some(3), Some(3), None, Some(4), Some(2)];
        let two = TreeNodeAdditions::new(2);
        let four = TreeNodeAdditions::new(4);
        let three_left = TreeNodeAdditions::with_children(3, four, two);
        let three = TreeNodeAdditions::with_children(3, three_left, None);
        let result = Solution::good_nodes(three);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        // let items = vec![Some(1)];
        let one = TreeNodeAdditions::new(1);
        let result = Solution::good_nodes(one);
        assert_eq!(result, 1);
    }

}
