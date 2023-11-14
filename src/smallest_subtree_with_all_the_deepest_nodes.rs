use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, the depth of each node is the shortest distance to the root.
///
/// Return the smallest subtree such that it contains all the deepest nodes in the original tree.
///
/// A node is called the deepest if it has the largest depth possible among any node in the entire
/// tree.
///
/// The subtree of a node is a tree consisting of that node, plus the set of all descenedants of
/// that node.
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

    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
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
        let result = Solution::subtree_with_all_deepest(root);
        assert_tree!(result, "[2,7,4]");
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::subtree_with_all_deepest(root);
        assert_tree!(result, "[1]");
    }

    #[test]
    fn example_3() {
        let root = tree!("[0,1,3,null,2]");
        let result = Solution::subtree_with_all_deepest(root);
        assert_tree!(result, "[2]");
    }

}
