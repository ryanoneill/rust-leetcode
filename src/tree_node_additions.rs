use crate::tree_node::TreeNode;
use std::cell::Ref;
use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait TreeNodeAdditions {

    fn new(val: i32) -> Self;
    fn with_children(val: i32, left: Self, right: Self) -> Self;

    fn clone_left(&self) -> Self;
    fn clone_right(&self) -> Self;

    /// For use with Binary Trees of Nodes.
    fn find_children(&self) -> HashMap<i32, HashSet<i32>>;
    fn find_children_worker(&self, children: &mut HashMap<i32, HashSet<i32>>);

    /// For use with Binary Trees of Nodes.
    fn find_parents(&self) -> HashMap<i32, i32>;
    fn find_parents_worker(&self, parents: &mut HashMap<i32, i32>);

    fn get_value(&self) -> Option<i32>;
    fn set_value(&mut self, value: i32);

    fn in_order(&self) -> Vec<i32>;
    fn in_order_worker<F: FnMut(&Ref<TreeNode>)>(&self, f: &mut F);
    fn leaf_value_sequence(&self) -> Vec<i32>;
    fn is_leaf(&self) -> bool;

    fn max_depth(&self) -> usize;
    fn min_depth(&self) -> usize;

    fn has_left(&self) -> bool;
    fn has_right(&self) -> bool;

    fn set_left(&mut self, value: Self);
    fn set_right(&mut self, value: Self);

    fn take_left(&mut self) -> Self;
    fn take_right(&mut self) -> Self;

}

impl TreeNodeAdditions for Option<Rc<RefCell<TreeNode>>> {
    fn new(val: i32) -> Self {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn with_children(val: i32, left: Self, right: Self) -> Self {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn clone_left(&self) -> Self {
        self.as_ref().and_then(|rc| rc.borrow().left.clone())
    }

    fn clone_right(&self) -> Self {
        self.as_ref().and_then(|rc| rc.borrow().right.clone())
    }

    fn find_children(&self) -> HashMap<i32, HashSet<i32>> {
        let mut children = HashMap::new();
        self.find_children_worker(&mut children);
        children
    }

    fn find_children_worker(&self, children: &mut HashMap<i32, HashSet<i32>>) {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                let val = node.val;
                let left_value = node.left.get_value();
                match left_value {
                    Some(left_val) => {
                        children
                            .entry(val)
                            .or_insert(HashSet::new())
                            .insert(left_val);
                        node.left.find_children_worker(children);
                    }
                    None => {}
                }
                let right_value = node.right.get_value();
                match right_value {
                    Some(right_val) => {
                        children
                            .entry(val)
                            .or_insert(HashSet::new())
                            .insert(right_val);
                        node.right.find_children_worker(children);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    fn find_parents(&self) -> HashMap<i32, i32> {
        let mut parents = HashMap::new();
        self.find_parents_worker(&mut parents);
        parents
    }

    fn find_parents_worker(&self, parents: &mut HashMap<i32, i32>) {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                let val = node.val;
                let left_value = node.left.get_value();
                match left_value {
                    Some(left_val) => {
                        parents.insert(left_val, val);
                        node.left.find_parents_worker(parents);
                    }
                    None => {}
                }
                let right_value = node.right.get_value();
                match right_value {
                    Some(right_val) => {
                        parents.insert(right_val, val);
                        node.right.find_parents_worker(parents);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    fn get_value(&self) -> Option<i32> {
        self.as_ref().map(|rc| {
            let node = rc.borrow();
            node.val
        })
    }

    fn set_value(&mut self, value: i32) {
        self.as_ref().map(|rc| {
            rc.borrow_mut().val = value;
        });
    }

    fn in_order_worker<F: FnMut(&Ref<TreeNode>)>(&self, f: &mut F) {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                if node.left.is_some() {
                    node.left.in_order_worker(f);
                }
                f(&node);
                if node.right.is_some() {
                    node.right.in_order_worker(f);
                }
            }
            None => {
                // do nothing
            }
        }
    }

    fn in_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.in_order_worker(&mut |node| {
            result.push(node.val);
        });
        result
    }

    fn is_leaf(&self) -> bool {
        self.as_ref()
            .map(|rc| {
                let node = rc.borrow();
                node.left.is_none() && node.right.is_none()
            })
            .unwrap_or(false)
    }

    fn leaf_value_sequence(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.in_order_worker(&mut |node| {
            if node.left.is_none() && node.right.is_none() {
                result.push(node.val);
            }
        });
        result
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
        if self.is_none() {
            0
        } else if self.is_leaf() {
            1
        } else {
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

    fn has_left(&self) -> bool {
        self.as_ref().map(|rc| {
            rc.borrow().left.is_some()
        }).unwrap_or_default()
    }

    fn has_right(&self) -> bool {
        self.as_ref().map(|rc| {
            rc.borrow().right.is_some()
        }).unwrap_or_default()
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

    fn take_left(&mut self) -> Self {
        let mut result = None;
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                result = node.left.take();
            }
            None => {
                // do nothing
            }
        }
        result
    }

    fn take_right(&mut self) -> Self {
        let mut result = None;
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                result = node.right.take();
            }
            None => {
                // do nothing
            }
        }
        result
    }

}
