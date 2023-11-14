use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;

/// Given the `root` of a binary tree, return the preorder traversal of its nodes' values.
struct Solution; 

impl Solution {

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();

        Self::preorder_worker(root, &mut results);

        results
    }

    fn preorder_worker(node: Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
        if node.is_some() {
            let value = node.get_value().unwrap();
            results.push(value);

            Self::preorder_worker(node.clone_left(), results);
            Self::preorder_worker(node.clone_right(), results);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,null,2,3]");
        let result = Solution::preorder_traversal(root);
        assert_eq!(result, vec![1,2,3]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[]");
        let result = Solution::preorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[1]");
        let result = Solution::preorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

}
