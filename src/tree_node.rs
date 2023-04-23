use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // TODO: Implement
    pub fn from_vec(_items: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

    pub fn to_vec(_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        vec![]
    }

}
