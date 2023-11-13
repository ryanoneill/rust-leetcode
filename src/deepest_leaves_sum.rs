use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the sum of values of its
/// deepest leaves.
struct Solution;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            let mut values = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(root.clone());

            while !queue.is_empty() {
                let len = queue.len();
                values = Vec::new();

                for _ in 0..len {
                    let item = queue.pop_front().unwrap();
                    match item {
                        Some(rc) => {
                            let node = rc.borrow();
                            values.push(node.val);
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
            }

            values.iter().sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,4,5,null,6,7,null,null,null,null,8]");
        let result = Solution::deepest_leaves_sum(root);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_2() {
        let root = tree!("[6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]");
        let result = Solution::deepest_leaves_sum(root);
        assert_eq!(result, 19);
    }

}
