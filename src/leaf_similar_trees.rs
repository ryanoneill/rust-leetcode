use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Consider all the leaves of a binary tree, from left to right order, the
/// values of those leaves form a leaf value sequence.
///
/// For example, in the given tree above, the leaft value sequence is
/// `(6, 7, 4, 9, 8)`.
///
/// Two binary trees are considered leaf-similar if their leaf value sequence
/// is the same.
///
/// Return `true` if and only if the two given trees with head nodes `root1`
/// and `root2` are leaf-similar.
struct Solution;

impl Solution {

    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let leaf1 = root1.leaf_value_sequence();
        let leaf2 = root2.leaf_value_sequence();

        leaf1 == leaf2
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use crate::tree_node_additions::TreeNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let codec = Codec::new();
        let data1 = "[3,5,1,6,2,9,8,null,null,7,4]".to_string();
        let root1 = codec.deserialize(data1);
        let data2 = "[3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]".to_string();
        let root2 = codec.deserialize(data2);
        let result = Solution::leaf_similar(root1, root2);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let codec = Codec::new();
        let data1 = "[1,2,3]".to_string();
        let root1 = codec.deserialize(data1);
        let data2 = "[1,3,2]".to_string();
        let root2 = codec.deserialize(data2);
        let result = Solution::leaf_similar(root1, root2);
        assert!(!result);
    }

}
