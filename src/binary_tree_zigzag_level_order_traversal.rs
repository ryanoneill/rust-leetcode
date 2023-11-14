use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the zigzag level order traversal
/// of its nodes' values. (i.e., from left to right, then right to left for the
/// next level and alternate between).
struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            vec![]
        } else {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let mut queue = VecDeque::new();
            let mut forward = true;
            queue.push_back(root.clone());

            while !queue.is_empty() {
                let len = queue.len();
                let mut row = Vec::new();

                for _ in 0..len {
                    let item = queue.pop_front().unwrap();
                    match item {
                        Some(rc) => {
                            let node = rc.borrow();
                            row.push(node.val);
                            if node.left.is_some() {
                                queue.push_back(node.left.clone());
                            }
                            if node.right.is_some() {
                                queue.push_back(node.right.clone());
                            }
                        }
                        None => {
                            // do nothing
                        }
                    }
                }
                if forward {
                    result.push(row);
                } else {
                    result.push(row.into_iter().rev().collect());
                }
                forward = !forward;
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![3], vec![20, 9], vec![15, 7]]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[]");
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}
