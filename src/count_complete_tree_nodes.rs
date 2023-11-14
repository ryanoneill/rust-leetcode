use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a complete binary tree, return the number of nodes in the tree.
///
/// According to `Wikipedia`, every level, except possibly the last, is completely filled in a
/// complete binary tree, and all nodes in the last level are as far left as possible. It can have
/// between `1` and `2^h` nodes inclusive at the last level `h`.
///
/// Design an algorithm that runs in less than `O(n)` time complexity.
struct Solution;

impl Solution {

    fn left_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                1 + Self::left_depth(&node.left)
            }
            None => {
                0
            }
        }
    }

    fn right_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                1 + Self::right_depth(&node.right)
            }
            None => {
                0
            }
        }

    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::worker(&root)
    }

    pub fn worker(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let left = Self::left_depth(&node.left);
                let right = Self::right_depth(&node.left);
                if left == right {
                    (1 << left) + Self::worker(&node.right)
                } else {
                    (1 << right) + Self::worker(&node.left)
                }
            }
            None => {
                0
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,4,5,6]");
        let result = Solution::count_nodes(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let root = tree!("[]");
        let result = Solution::count_nodes(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let root = tree!("[1]");
        let result = Solution::count_nodes(root);
        assert_eq!(result, 1);
    }

}
