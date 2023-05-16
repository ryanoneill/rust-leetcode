use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the roots of two binary trees `p` and `q`, write a function to check
/// if they are the same or not.
///
/// Two binary trees are considered the same if they are structurally identical, and
/// the nodes have the same value.
struct Solution;

impl Solution {

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p_rc), Some(q_rc)) => {
                let p_node = p_rc.borrow();
                let q_node = q_rc.borrow();

                let mut result = p_node.val == q_node.val;
                result = result && Self::is_same_tree(p_node.left.clone(), q_node.left.clone());
                result = result && Self::is_same_tree(p_node.right.clone(), q_node.right.clone());

                result
            }
            (None, None) => true,
            _ => false
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let p_data = "[1,2,3]".to_string();
        let q_data = "[1,2,3]".to_string();
        let codec = Codec::new();
        let p = codec.deserialize(p_data);
        let q = codec.deserialize(q_data);
        let result = Solution::is_same_tree(p, q);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let p_data = "[1,2]".to_string();
        let q_data = "[1,null,2]".to_string();
        let codec = Codec::new();
        let p = codec.deserialize(p_data);
        let q = codec.deserialize(q_data);
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let p_data = "[1,2,1]".to_string();
        let q_data = "[1,1,2]".to_string();
        let codec = Codec::new();
        let p = codec.deserialize(p_data);
        let q = codec.deserialize(q_data);
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

}
