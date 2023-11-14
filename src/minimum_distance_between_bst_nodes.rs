use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a Binary Search Tree (BST), return the minimum distance
/// between the values of any two different nodes in the tree.
struct Solution;

impl Solution {
    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, previous: &mut Option<i32>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut result = i32::max_value();

                if node.left.is_some() {
                    result = result.min(Self::worker(&node.left, previous));
                }
                previous.map(|p| {
                    result = result.min(node.val - p);
                });
                *previous = Some(node.val);
                if node.right.is_some() {
                    result = result.min(Self::worker(&node.right, previous));
                }
                result
            }
            None => i32::max_value(),
        }
    }

    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut previous = None;
        Self::worker(&root, &mut previous)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[4,2,6,1,3]");
        let result = Solution::min_diff_in_bst(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,0,48,null,null,12,49]");
        let result = Solution::min_diff_in_bst(root);
        assert_eq!(result, 1);
    }
}
