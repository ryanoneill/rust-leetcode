use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// You are given the `root` node of a binary tree (BST) and a `value` to
/// insert into the tree. Return the root node of the BST after the insertion.
/// It is guaranteed that the new value does not exist in the origin BST.
///
/// Notice that there may exist multiple valid ways for the insertion, as long
/// as the tree remains a BST after insertion. You can return any of them.
struct Solution;

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let result = root.clone();
        match root {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                let value = node.val;
                if val < value {
                    if node.left.is_none() {
                        node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        Self::insert_into_bst(node.left.clone(), val);
                    }
                } else {
                    // greater*
                    // *It is guaranteed in the problem that the value is not in the BST.
                    if node.right.is_none() {
                        node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        Self::insert_into_bst(node.right.clone(), val);
                    }
                }

                result
            }
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_node_additions::TreeNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[4,2,7,1,3]");
        let result = Solution::insert_into_bst(root, 5);
        let values = result.in_order();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[40,20,60,10,30,50,70]");
        let result = Solution::insert_into_bst(root, 25);
        let values = result.in_order();
        assert_eq!(values, vec![10, 20, 25, 30, 40, 50, 60, 70]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[4,2,7,1,3,null,null,null,null,null,null]");
        let result = Solution::insert_into_bst(root, 5);
        let values = result.in_order();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 7]);
    }
}
