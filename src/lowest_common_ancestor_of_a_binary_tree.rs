use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given a binary tree, find the lowest common ancestor (LCA) of two given
/// nodes in the tree.
///
/// Accoring to the definition of LCA on Wikipedia: "The lowest common ancestor
/// is defined between two nodes `p` and `q` as the lowest node in `T` that has
/// both `p` and `q` as descendants (where we allow a node to be a descendant
/// of itself)."
///
struct Solution;

impl Solution {

    fn find_path(
        root: &Option<Rc<RefCell<TreeNode>>>,
        value: i32,
        path: VecDeque<Option<Rc<RefCell<TreeNode>>>>,
    ) -> VecDeque<Option<Rc<RefCell<TreeNode>>>> {
        let mut result: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut path = path;
        path.push_back(root.clone());
        match root {
            Some(rc) => {
                let node = rc.borrow();
                if value == node.val {
                    result = path.clone();
                } else {
                    if node.left.is_some() {
                        result = Self::find_path(&node.left, value, path.clone());
                    }
                    if result.len() == 0 && node.right.is_some() {
                        result = Self::find_path(&node.right, value, path.clone());
                    }
                }
            }
            None => { }
        }

        result
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_value = p.map(|n| n.borrow().val).unwrap_or_default();
        let q_value = q.map(|n| n.borrow().val).unwrap_or_default();

        let mut p_path = Self::find_path(&root, p_value, VecDeque::new());
        let mut q_path = Self::find_path(&root, q_value, VecDeque::new());

        let mut result = None;
        while !p_path.is_empty() && !q_path.is_empty() {
            let p_ancestor = p_path.pop_front().unwrap();
            let q_ancestor = q_path.pop_front().unwrap();
            if p_ancestor == q_ancestor {
                result = p_ancestor;
            }
        }
            
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;

    #[test]
    fn example_1() {
        let data = "[3,5,1,6,2,0,8,null,null,7,4]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let p = root.clone_left();
        assert_eq!(p.get_value(), Some(5)); // Double check
        let q = root.clone_right();
        assert_eq!(q.get_value(), Some(1)); // Double check
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.get_value(), Some(3));
    }

    #[test]
    fn example_2() {
        let data = "[3,5,1,6,2,0,8,null,null,7,4]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let p = root.clone_left();
        assert_eq!(p.get_value(), Some(5));
        let q = root.clone_left().clone_right().clone_right();
        assert_eq!(q.get_value(), Some(4));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.get_value(), Some(5));
    }

    #[test]
    fn example_3() {
        let data = "[1,2]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let p = root.clone();
        assert_eq!(p.get_value(), Some(1));
        let q = root.clone_left();
        assert_eq!(q.get_value(), Some(2));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.get_value(), Some(1));
    }

}
