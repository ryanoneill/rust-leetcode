use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait TreeNodeAdditions {

    fn new(val: i32) -> Self;
    fn with_children(val: i32, left: Self, right: Self) -> Self;

    fn get_value(&self) -> Option<i32>;
    fn is_leaf(&self) -> bool;
    fn max_depth(&self) -> usize;
    fn min_depth(&self) -> usize;
    fn set_left(&mut self, value: Self);
    fn set_right(&mut self, value: Self);

}

impl TreeNodeAdditions for Option<Rc<RefCell<TreeNode>>> {

    fn new(val: i32) -> Self {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn with_children(val: i32, left: Self, right: Self) -> Self {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn get_value(&self) -> Option<i32> {
        self.as_ref().map(|rc| {
            let node = rc.borrow();
            node.val
        })
    }

    fn is_leaf(&self) -> bool {
        self.as_ref().map(|rc| {
            let node = rc.borrow();
            node.left.is_none() && node.right.is_none()
        }).unwrap_or(false)
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

    fn min_depth(&self) -> usize {
        if self.is_none() { 0 }
        else if self.is_leaf() { 1 }
        else {
            let rc = self.as_ref().unwrap();
            let node = rc.borrow();
            if node.left.is_some() && node.right.is_some() {
                1 + min(node.left.min_depth(), node.right.min_depth())
            } else if node.left.is_some() {
                1 + node.left.min_depth()
            } else {
                1 + node.right.min_depth()
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

}
