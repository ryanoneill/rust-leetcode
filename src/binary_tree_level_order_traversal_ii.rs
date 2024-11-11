use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the bottom-up level order traversal of its nodes'
/// values. (i.e. from left to right, level by level from leaf to root).
struct Solution;

impl Solution {

    fn worker(
        results: &mut Vec<Vec<i32>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
        level: usize
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let n = results.len();
                if level >= n {
                    results.push(vec![node.val]);
                } else {
                    results[level].push(node.val);
                }
                if node.left.is_some() {
                    Self::worker(results, &node.left, level + 1);
                }
                if node.right.is_some() {
                    Self::worker(results, &node.right, level + 1);
                }
            }
            None => { }
        }
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        Self::worker(&mut results, &root, 0);
        results.reverse();

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::level_order_bottom(root);
        assert_eq!(result, vec![
            vec![15,7],
            vec![9,20],
            vec![3],
        ]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::level_order_bottom(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[]");
        let result = Solution::level_order_bottom(root);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

}
