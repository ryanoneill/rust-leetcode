use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;

/// Given the `root` of a binary tree, return the postorder traversal of its nodes' values.
struct Solution; 

impl Solution {

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();

        Self::postorder_worker(root, &mut results);

        results
    }

    fn postorder_worker(node: Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
        if node.is_some() {
            Self::postorder_worker(node.clone_left(), results);
            Self::postorder_worker(node.clone_right(), results);

            let value = node.get_value().unwrap();
            results.push(value);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,null,2,3]");
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![3,2,1]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[]");
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[1]");
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

}
