use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;

/// You are given two binary trees `root1` and `root2`.
///
/// Imagine that when you put one of them to cover the other, some nodes of the two trees are
/// overlapped while the others are not. You need to merge the two trees into a new binary tree.
/// The merge rule is that if two nodes overlap, then sum node values up as the new value of the
/// merged node. Otherwise, the NOT null node will be used as the node of the new tree.
///
/// Return the merged tree.
///
/// Note: The merging process must start from the root nodes of both trees.
struct Solution;

impl Solution {

    fn worker(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(rc1), Some(rc2)) => {
                let node1 = rc1.borrow();
                let node2 = rc2.borrow();
                let sum = node1.val + node2.val;

                let mut result: Option<Rc<RefCell<TreeNode>>> = TreeNodeAdditions::new(sum);
                result.set_left(Self::worker(&node1.left, &node2.left));
                result.set_right(Self::worker(&node1.right, &node2.right));
                result
            }
            (Some(rc1), None) => {
                let node1 = rc1.borrow();
                let mut result: Option<Rc<RefCell<TreeNode>>> = TreeNodeAdditions::new(node1.val);
                result.set_left(Self::worker(&node1.left, &None));
                result.set_right(Self::worker(&node1.right, &None));
                result
            }
            (None, Some(rc2)) => {
                let node2 = rc2.borrow();
                let mut result: Option<Rc<RefCell<TreeNode>>> = TreeNodeAdditions::new(node2.val);
                result.set_left(Self::worker(&None, &node2.left));
                result.set_right(Self::worker(&None, &node2.right));
                result
            }
            (None, None) => {
                None
            }
        }
    }

    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::worker(&root1, &root2)
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let root1_data = str!("[1,3,2,5]");
        let root2_data = str!("[2,1,3,null,4,null,7]");
        let codec = Codec::new();
        let root1 = codec.deserialize(root1_data);
        let root2 = codec.deserialize(root2_data);
        let result = Solution::merge_trees(root1, root2);
        let items = codec.serialize(result);
        assert_eq!(items, "[3,4,5,5,4,null,7]");
    }

    #[test]
    fn example_2() {
        let root1_data = str!("[1]");
        let root2_data = str!("[1,2]");
        let codec = Codec::new();
        let root1 = codec.deserialize(root1_data);
        let root2 = codec.deserialize(root2_data);
        let result = Solution::merge_trees(root1, root2);
        let items = codec.serialize(result);
        assert_eq!(items, "[2,2]");
    }

}
