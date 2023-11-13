use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the lowest common ancestor of its deepest leaves.
///
/// Recall that:
///
/// * The node of a binary tree is a leaf if and only if it has no children
///
/// * The depth of the root of the tree is `0`. If the depth of a node is `d`, the depth of each of
///   its children is `d + 1`.
///
/// * The lowest common ancestor of a set `S` of nodes, is the node `A` with the largest depth such
///   that every node in `S` is in the subtree with root `A`.
struct Solution;

impl Solution {

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let left = Self::height(&node.left);
                let right = Self::height(&node.right);
                let result = 1 + left.max(right);
                result
            }
            None => {
                0
            }
        }
    }

    fn worker(
        root: &Option<Rc<RefCell<TreeNode>>>,
        height: usize,
        current: usize,
        path: Vec<Option<Rc<RefCell<TreeNode>>>>,
        results: &mut Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let current = current + 1;
                let mut path = path;
                path.push(root.clone());
                if current == height {
                    results.push(path);
                } else {
                    Self::worker(&node.left, height, current, path.clone(), results);
                    Self::worker(&node.right, height, current, path, results);
                }
            }
            None => {
                // do nothing
            }
        }
    }

    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result = None;

        let height = Self::height(&root);
        let mut results = Vec::new();
        Self::worker(&root, height, 0, Vec::new(), &mut results);

        let n = results.len();
        if n == 1 {
            result = results[0][height-1].clone();
        } else {
            let mut index = 0;
            while index < height {
                let mut done = false;
                let current = results[0][index].clone();
                for i in 1..n {
                    if results[i][index] != current {
                        done = true;
                        break;
                    }
                }

                if done {
                    break;
                } else {
                    result = current;
                    index += 1;
                }
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,5,1,6,2,0,8,null,null,7,4]");
        let result = Solution::lca_deepest_leaves(root);
        assert_tree!(result, "[2,7,4]");
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::lca_deepest_leaves(root);
        assert_tree!(result, "[1]");
    }

    #[test]
    fn example_3() {
        let root = tree!("[0,1,3,null,2]");
        let result = Solution::lca_deepest_leaves(root);
        assert_tree!(result, "[2]");
    }

}
