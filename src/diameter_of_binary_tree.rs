use crate::diameter_of_binary_tree;
use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

/// Given the `root` of a binary tree, return the length of the diameter of the
/// tree.
///
/// The diameter of a binary tree is the length of the longest path between any
/// two nodes in a tree. This pay may or may not pass through the `root`.
///
/// The length of a path between two nodes is represented by the number of
/// edges between them.
struct Solution;

impl Solution {

    fn diameter_of_binary_tree_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();

                let left_depth = node.left.max_depth() as i32;
                let right_depth = node.right.max_depth() as i32;
                let edges_between = left_depth + right_depth;

                let left_diameter = Self::diameter_of_binary_tree_ref(&node.left);
                let right_diameter = Self::diameter_of_binary_tree_ref(&node.right);

                max(edges_between, max(left_diameter, right_diameter))
            }
            None => { 0 }
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_of_binary_tree_ref(&root)
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[1,2,3,4,5]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let data = "[1,2]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 1);
    }

}

