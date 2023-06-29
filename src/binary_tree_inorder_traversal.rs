use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the inorder traversal of its nodes' values.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                Self::worker(&node.left, results);
                results.push(node.val);
                Self::worker(&node.right, results);
            }
            None => { } // do nothing
        }

    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();
        Self::worker(&root, &mut results);
        results
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1(){
        let data = "[1,null,2,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::inorder_traversal(root);
        assert_eq!(result, vec![1,3,2]);
    }

    #[test]
    fn example_2() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::inorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_3() {
        let data = "[1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::inorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

}
