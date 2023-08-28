use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    with_root: i32,
    without_root: i32,
}

impl Item {

    fn empty() -> Self {
        Self { with_root: -1001, without_root: -1001 }
    }

    fn new(with_root: i32, without_root: i32) -> Self {
        Self { with_root, without_root }
    }

    fn maximum(&self) -> i32 {
        self.with_root.max(self.without_root)
    }

}

/// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the
/// sequence has an edge connecting them. A node can only appear in the sequence at most once. Note
/// that the path does not need to pass through the root.
///
/// The path sum of a path is the sum of the node's values in the path.
///
/// Given the `root` of a binary tree, return the maximum path of any non-empty path.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>) -> Item {
        match root {
            Some(rc) => {
                let node = rc.borrow();

                let left_result = Self::worker(&node.left);
                let right_result = Self::worker(&node.right);

                let only_root = node.val;
                let left_with_root = node.val + left_result.with_root;
                let right_with_root = node.val + right_result.with_root;

                let with_root = only_root
                    .max(left_with_root)
                    .max(right_with_root);

                let both_with_root = node.val + left_result.with_root + right_result.with_root;
                let without_root = both_with_root
                    .max(left_result.maximum())
                    .max(right_result.maximum());

                Item::new(with_root, without_root)
            }
            None => {
                Item::empty()
            }
        }
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Self::worker(&root);
        result.maximum()
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = "[1,2,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(items);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let items = "[-10,9,20,null,null,15,7]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(items);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, 42);
    }

    #[test]
    fn example_single_negative() {
        let items = "[-3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(items);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, -3);
    }

}
