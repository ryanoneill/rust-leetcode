use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, determine if it is a valid binary search
/// tree (BST).
///
/// A valid BST is defined as follows:
/// * The left subtree of a node contains only nodes with keys less than the
///   node's key.
/// * The right subtree of a node contains only nodes with keys greater than
///   the node's key.
/// * Both the left and right subtrees must also be binary search trees.
struct Solution;

impl Solution {
    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, previous: &mut Option<i32>) -> bool {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut result = true;

                if result && node.left.is_some() {
                    result = result && Self::worker(&node.left, previous);
                }
                previous.map(|p| {
                    result = result && p < node.val;
                });
                *previous = Some(node.val);
                if result && node.right.is_some() {
                    result = result && Self::worker(&node.right, previous);
                }

                result
            }
            None => false,
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut previous = None;
        Self::worker(&root, &mut previous)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;

    #[test]
    fn example_1() {
        let data = "[2,1,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::is_valid_bst(root);
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let data = "[5,1,4,null,null,3,6]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::is_valid_bst(root);
        assert_eq!(result, false);
    }
}
