use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes
/// in the BST.
///
/// According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between
/// two nodes `p` and `q` as the lowest node in `T` that has both `p` and `q` as descendants (where
/// we allow a node to be a descendant of itself)."
struct Solution;

impl Solution {

    fn get_value(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                node.val
            }
            None => -1
        }
    }

    fn find_path_worker(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, results: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) {
        results.push(root.clone());
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val;
                if val < value {
                    Self::find_path_worker(&node.left, val, results);
                } else if val > value {
                    Self::find_path_worker(&node.right, val, results);
                } // val == value, stop here
            }
            None => { } // do nothing
        }
    }

    fn find_path(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut results = Vec::new();
        Self::find_path_worker(root, val, &mut results);
        results
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_value = Self::get_value(&p);
        let q_value = Self::get_value(&q);
        let p_path = Self::find_path(&root, p_value);
        let q_path = Self::find_path(&root, q_value);

        let mut result = None;
        let mut p_path_iter = p_path.into_iter();
        let mut q_path_iter = q_path.into_iter();

        loop {
            match (p_path_iter.next(), q_path_iter.next()) {
                (Some(p_node), Some(q_node)) if p_node == q_node => {
                    result = p_node;
                }
                (_, _) => {
                    break;
                }
            }
        }


        result
    }

}

#[cfg(test)]
mod tests {
    use crate::tree_node_additions::TreeNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[6,2,8,0,4,7,9,null,null,3,5]");
        let p = root.clone_left();
        let q = root.clone_right();
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.get_value().unwrap(), 6);
    }

    #[test]
    fn example_2() {
        let root = tree!("[2,1]");
        let p = root.clone();
        let q = root.clone_left();
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.get_value().unwrap(), 2);
    }

}
