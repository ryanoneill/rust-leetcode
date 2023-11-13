use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return all root-to-leaf paths in any order.
///
/// A leaf is a node with no children.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, path: Vec<i32>, results: &mut Vec<String>) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut path = path;
                let value = node.val;
                path.push(value);
                let is_leaf = node.left.is_none() && node.right.is_none();
                if is_leaf {
                    let n = path.len();
                    let mut s = path[0].to_string();
                    for i in 1..n {
                        let piece = path[i].to_string();
                        s = s + &"->" + &piece;
                    }
                    results.push(s);
                } else {
                    if node.left.is_some() {
                        Self::worker(&node.left, path.clone(), results);
                    }
                    if node.right.is_some() {
                        Self::worker(&node.right, path.clone(), results);
                    }
                }
            }
            None => { }
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut results = Vec::new();
        let path = Vec::new();
        Self::worker(&root, path, &mut results);
        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,null,5]");
        let result = Solution::binary_tree_paths(root);
        assert_eq!(result, vec!["1->2->5", "1->3"]);
    }


    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::binary_tree_paths(root);
        assert_eq!(result, vec!["1"]);
    }

}
