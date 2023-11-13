use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, flatten the tree into a "linked list".
///
/// * The "linked list" should use the same `TreeNode` class where the `right` child pointer points
///   to the next node in the list and the `left` child pointer is always `null`.
///
/// * The "linked list" should be in the same order as a pre-order traversal of the binary tree.
struct Solution;

impl Solution {

    fn has_right(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                node.right.is_some()
            }
            None => {
                false
            }
        }
    }

    fn clone_right(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                node.right.clone()
            }
            None => {
                None
            }
        }
    }

    fn set_right(root: &Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.right = right;
            }
            None => {
                // Do Nothing
            }
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                if node.right.is_some() {
                    Self::flatten(&mut node.right);
                }
                if node.left.is_some() {
                    let mut left = node.left.take();
                    Self::flatten(&mut left);
                    let right = node.right.take();
                    if right.is_some() {
                        let mut current = left.clone();
                        while Self::has_right(&current) {
                            current = Self::clone_right(&current);
                        }
                        Self::set_right(&current, right);
                    }
                    node.right = left;
                }

            }
            None => {
                // Do Nothing
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut root = tree!("[1,2,5,3,4,null,6]");
        Solution::flatten(&mut root);
        assert_tree!(root, "[1,null,2,null,3,null,4,null,5,null,6]");
    }

    #[test]
    fn example_2() {
        let mut root = tree!("[]");
        Solution::flatten(&mut root);
        assert_tree!(root, "[]");
    }

    #[test]
    fn example_3() {
        let mut root = tree!("[0]");
        Solution::flatten(&mut root);
        assert_tree!(root, "[0]");
    }

}
// 1. Write down the problem ✓
// 2. Clarify the problem space ✓
// ** The number of nodes in the tree is in the range `[0, 2000]`.
// ** Node.val >= -100
// ** Node.val <= 100
// ** Can you flatten the tree in-place (with `O(1)` extra space)?
// ** Input: Option<Rc<RefCell<TreeNode>>>
// ** Output: Option<Rc<RefCell<TreeNode>>> with right children only filled in, in order.
// 3. Write down the test cases
// ** Input: []                     Output: []
// ** Input: [1]                    Output: [1]
// ** Input: [1,2,3]                Output: [1,null,2,null,3]
// ** Input: [1,2,5,3,4,null,6]     Output: [1,null,2,null,3,null,4,null,5,null,6]
// 4. Design and write down the algorithm
// ** Does node have right child?
//    ** Yes: Flatten right child
//    ** No: Leave as is
// ** Does node have left child?
//    ** Yes: Flatten left child, add right child to end of left child, make flattened left the
//    right child.
//    ** No: Leave as is
// ** Done
