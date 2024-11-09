use crate::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

/// Given the `root` of a binary search tree (BST) with duplicates, return all the mode(s) (i.e.,
/// the most frequently occurred element) in it.
///
/// If the tree has more than one mode, return them in any order.
///
/// Assume a BST is defined as follows:
/// * The left subtree of a node contains only nodes with keys less than or equal to the node's
/// key.
///
/// * The right subtree of a node contains only nodes with keys greater than or equal to the node's
/// key.
///
/// * Both the left and right subtrees must also be binary search trees.
struct Solution;

impl Solution {

    fn worker(
        results: &mut Vec<i32>,
        maximum: &mut usize,
        count: &mut usize,
        last: &mut i32,
        root: &Option<Rc<RefCell<TreeNode>>>
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                if node.left.is_some() {
                    Self::worker(results, maximum, count, last, &node.left);
                }
                let value = node.val;
                if *last == value {
                    *count += 1;
                } else {
                    *count = 1;
                }
                if *count == *maximum {
                    results.push(value);
                } else if *count > *maximum {
                    results.clear();
                    results.push(value);
                    *maximum = *count;
                }
                *last = value;
                if node.right.is_some() {
                    Self::worker(results, maximum, count, last, &node.right);
                }
            }
            None => { }
        }

    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();
        let mut maximum = 0;
        let mut count = 0;
        let mut last = i32::MIN;

        Self::worker(
            &mut results,
            &mut maximum,
            &mut count,
            &mut last,
            &root);

        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,null,2,2]");
        let result = Solution::find_mode(root);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[0]");
        let result = Solution::find_mode(root);
        assert_eq!(result, vec![0]);
    }

}
