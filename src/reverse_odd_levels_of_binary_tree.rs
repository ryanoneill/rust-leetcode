use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a perfect binary tree, reverse the node values at each odd level of the
/// tree.
///
/// * For example, suppose the node values at level 3 are `[2,1,3,4,7,11,29,18]`, then it should
/// become `[18,29,11,7,4,3,1,2]`.
///
/// Return the root of the reversed tree.
///
/// A binary tree is perfect if all parent nodes have two children and all leaves are on the same
/// level.
///
/// The level of a node is the number of edges along the path between it and the root node.
struct Solution;

impl Solution {

    fn get_value_or_zero(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let inner = rc.borrow();
                inner.val
            }
            None => {
                0
            }
        }
    }

    fn set_value(root: &Option<Rc<RefCell<TreeNode>>>, value: i32) {
        match root {
            Some(rc) => {
                let mut inner = rc.borrow_mut();
                inner.val = value;
            }
            None => {}
        }
    }

    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut level = 0;

        let mut nodes = VecDeque::new();
        nodes.push_back(root.clone());
        let mut values = Vec::new();

        while !nodes.is_empty() {
            let n = nodes.len();
            if level % 2 == 1 {
                for i in 0..n {
                    let value = Self::get_value_or_zero(&nodes[i]);
                    values.push(value);
                }
            }
            for _ in 0..n {
                let node = nodes.pop_front().unwrap();
                if level % 2 == 1 {
                    let value = values.pop().unwrap();
                    Self::set_value(&node, value);
                }
                match node {
                    Some(rc) => {
                        let inner = rc.borrow();
                        nodes.push_back(inner.left.clone());
                        nodes.push_back(inner.right.clone());
                    }
                    None => { }
                }
            }
            level += 1;
        }

        root
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[2,3,5,8,13,21,34]");
        let result = Solution::reverse_odd_levels(root);
        assert_tree!(result, "[2,5,3,8,13,21,34]");
    }

    #[test]
    fn example_2() {
        let root = tree!("[7,13,11]");
        let result = Solution::reverse_odd_levels(root);
        assert_tree!(result, "[7,11,13]");
    }

    #[test]
    fn example_3() {
        let root = tree!("[0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]");
        let result = Solution::reverse_odd_levels(root);
        assert_tree!(result, "[0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]");
    }

}
