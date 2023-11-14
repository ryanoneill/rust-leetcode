use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, find the maximum value `v` for which
/// there exist different nodes `a` and `b` where `v = |a.val - b.val|` and
/// `a` is an ancestor of `b`.
///
/// A node `a` is an ancestor of `b` if either: any child of `a` is equal
/// to `b` or any child of `a` is an ancestor of `b`.
struct Solution;

impl Solution {

    fn max_ancestor_diff_high_low(
        root: Option<Rc<RefCell<TreeNode>>>,
        highest: i32,
        lowest: i32,
    ) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val;
                let new_highest = value.max(highest);
                let new_lowest = value.min(lowest);
                let diff = new_highest - new_lowest;

                let left =
                    Self::max_ancestor_diff_high_low(node.left.clone(), new_highest, new_lowest);
                let right =
                    Self::max_ancestor_diff_high_low(node.right.clone(), new_highest, new_lowest);

                diff.max(left).max(right)
            }
            None => 0,
        }
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            Self::max_ancestor_diff_high_low(root, i32::min_value(), i32::max_value())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[8,3,10,1,6,null,14,null,null,4,7,13]");
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,null,2,null,0,3]");
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(result, 3);
    }
}
