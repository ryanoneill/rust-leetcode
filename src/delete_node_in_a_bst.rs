use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given a root node reference of a BST and a key, delete the node with the
/// given key in the BST. Return the root node reference (possibly updated) of
/// the BST.
///
/// Basically, the deletion can be divided into two stages:
///
/// Search for a node to remove.
///
/// If the node is found, delete the node.
struct Solution;

impl Solution {

    // 1. If the target node has no child, we can remove the node.
    // 2. If the target node has one child, we can use its child to replace itself.
    // 3. If the target node has two children, replace the node with its in-order
    //    sucessor or predecessor node and delete that node.

    fn find_leftest_child(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let result = root.clone();
        match root {
            Some(rc) => {
                let node = rc.borrow();
                if node.left.is_none() {
                    result
                } else {
                    Self::find_leftest_child(&node.left)
                }
            }
            None => {
                result
            }
        }
    }

    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let value = root.get_value();
        match value {
            Some(val) => {
                if val == key {
                    if !root.has_left() {
                        root.take_right()
                    } else if !root.has_right() {
                        root.take_left()
                    } else {
                        let right = root.take_right();
                        let successor = Self::find_leftest_child(&right);
                        let s_value = successor.get_value().unwrap();
                        root.set_value(s_value);
                        root.set_right(Self::delete_node(right, s_value));
                        root
                    }
                } else if key < val {
                    let left = root.take_left();
                    let result = Self::delete_node(left, key);
                    root.set_left(result);
                    root
                } else {
                    let right = root.take_right();
                    let result = Self::delete_node(right, key);
                    root.set_right(result);
                    root
                }
            }
            None => {
                None
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[5,3,6,2,4,null,7]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let key = 3;
        let result = Solution::delete_node(root, key);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[5,4,6,2,null,null,7]");
    }

    #[test]
    fn example_2() {
        let data = "[5,3,6,2,4,null,7]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let key = 0;
        let result = Solution::delete_node(root, key);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[5,3,6,2,4,null,7]");
    }

    #[test]
    fn example_3() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let key = 0;
        let result = Solution::delete_node(root, key);
        let result_data = codec.serialize(result);
        assert_eq!(result_data, "[]");
    }

}
