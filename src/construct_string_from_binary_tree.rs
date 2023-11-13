use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, construct a string consisting of parenthesis and integers
/// from a binary tree with the preorder traversal way, and return it.
///
/// Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship
/// between the string and the original binary tree.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val;
                result.push_str(&value.to_string());
                if node.right.is_some() {
                    result.push('(');
                    Self::worker(&node.left, result);
                    result.push(')');

                    result.push('(');
                    Self::worker(&node.right, result);
                    result.push(')');
                } else if node.left.is_some() {
                    result.push('(');
                    Self::worker(&node.left, result);
                    result.push(')');
                }
            }
            None => {
            }
        }
    }

    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::from("");
        Self::worker(&root, &mut result);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,4]");
        let result = Solution::tree2str(root);
        assert_eq!(result, "1(2(4))(3)");
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2,3,null,4]");
        let result = Solution::tree2str(root);
        assert_eq!(result, "1(2()(4))(3)");
    }

}
