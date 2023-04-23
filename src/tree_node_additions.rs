use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait TreeNodeAdditions {

    fn new(value: i32) -> Self;
    fn from_vec(items: Vec<Option<i32>>) -> Self;

    fn set_left(&mut self, value: Self);
    fn set_right(&mut self, value: Self);
    fn to_vec(&self) -> Vec<Option<i32>>;

}

impl TreeNodeAdditions for Option<Rc<RefCell<TreeNode>>> {

    fn new(value: i32) -> Self {
        Some(Rc::new(RefCell::new(TreeNode::new(value))))
    }

    // TODO: Implement
    fn from_vec(_items: Vec<Option<i32>>) -> Self {
        None
    }

    fn set_left(&mut self, value: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.left = value;
            }
            None => {
                // do nothing
            }
        }
    }

    fn set_right(&mut self, value: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.right = value;
            }
            None => {
                // do nothing
            }
        }
    }

    // TODO: Implement
    fn to_vec(&self) -> Vec<Option<i32>> {
        vec![]
    }

}
