use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the average value of the nodes
/// on each level in the form of an array. Answers within `10^-5` of the actual
/// answer will be accepted.
struct Solution;

impl Solution {

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root.clone());
        while !queue.is_empty() {
            let n = queue.len();
            let mut sum = 0.0;
            for _ in 0..n {
                let item = queue.pop_front().unwrap();
                match item {
                    Some(rc) => {
                        let node = rc.borrow();
                        sum += node.val as f64;
                        if node.left.is_some() {
                            queue.push_back(node.left.clone());
                        }
                        if node.right.is_some() {
                            queue.push_back(node.right.clone());
                        }
                    }
                    None => { }
                }
            }
            let average = sum / n as f64;
            result.push(average);
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![3.00000,14.50000,11.00000]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[3,9,20,15,7]");
        let result = Solution::average_of_levels(root);
        assert_eq!(result, vec![3.00000,14.50000,11.00000]);
    }

}
