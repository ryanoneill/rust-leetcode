use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given two integer arrays `preorder` and `inorder` where `preorder` is the preorder traversal of
/// a binary tree and `inorder` is the inorder traversal of the same tree, construct and return the
/// binary tree.
struct Solution;

impl Solution {

    fn worker(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let p_len = preorder.len();
        if p_len == 0 { 
            None
        } else {
            let val = preorder[0];
            let mid = inorder.iter().position(|&i| i == val).unwrap();

            let left_preorder = &preorder[1..mid+1];
            let right_preorder = &preorder[mid+1..];
            let left_inorder = &inorder[..mid];
            let right_inorder = &inorder[mid+1..];

            let mut root = TreeNode::new(preorder[0]);
            root.left = Self::worker(left_preorder, left_inorder);
            root.right = Self::worker(right_preorder, right_inorder);
            Some(Rc::new(RefCell::new(root)))
        }
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::worker(&preorder, &inorder)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let preorder = vec![3,9,20,15,7];
        let inorder = vec![9,3,15,20,7];
        let result = Solution::build_tree(preorder, inorder);
        assert_tree!(result, "[3,9,20,null,null,15,7]");
    }

    #[test]
    fn example_2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let result = Solution::build_tree(preorder, inorder);
        assert_tree!(result, "[-1]");
    }

}
