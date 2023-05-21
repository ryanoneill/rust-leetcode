use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, invert the tree, and return its root.
struct Solution;

impl Solution {

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        if root.is_some() {
            let mut left = root.take_left();
            let mut right = root.take_right();

            left = Self::invert_tree(left);
            right = Self::invert_tree(right);

            root.set_left(right);
            root.set_right(left);
        }
        root
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[4,2,7,1,3,6,9]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::invert_tree(root);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[4,7,2,9,6,3,1]");
    }

    #[test]
    fn example_2() {
        let data = "[2,1,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::invert_tree(root);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[2,3,1]");
    }

    #[test]
    fn example_3() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::invert_tree(root);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[]");
    }

}
