use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait TreeNodeAdditions {

    fn new(val: i32) -> Self;
    fn with_children(val: i32, left: Self, right: Self) -> Self;
    fn from_vec(items: Vec<Option<i32>>) -> Self;

    fn max_depth(&self) -> usize;
    fn set_left(&mut self, value: Self);
    fn set_right(&mut self, value: Self);
    fn to_vec(&self) -> Vec<Option<i32>>;

}

impl TreeNodeAdditions for Option<Rc<RefCell<TreeNode>>> {

    fn new(val: i32) -> Self {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn with_children(val: i32, left: Self, right: Self) -> Self {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    // TODO: Implement
    fn from_vec(_items: Vec<Option<i32>>) -> Self {
        None
    }

    fn max_depth(&self) -> usize {
        match self {
            None => 0,
            Some(rc) => {
                let node = rc.borrow();
                let left = node.left.max_depth();
                let right = node.right.max_depth();

                1 + max(left, right)
            }
        }
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
