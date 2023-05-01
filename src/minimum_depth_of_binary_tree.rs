use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

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
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_1() {
        let data = "[3,9,20,null,null,15,7]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::min_depth(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let data = "[2,null,3,null,4,null,5,null,6]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::min_depth(root);
        assert_eq!(result, 5);
    }

    #[test]
    fn root_only() {
        let root = TreeNodeAdditions::new(10);
        let result = Solution::min_depth(root);
        assert_eq!(result, 1);
    }
}
