use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

/// Given the `root` of a Binary Search Tree (BST), return the minimum absolute
/// difference between the two values of any two different nodes in the tree.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, previous: &mut Option<i32>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut result = i32::max_value();

                if node.left.is_some() {
                    result = min(result, Self::worker(&node.left, previous));
                }
                previous.map(|p| {
                    result = min(result, node.val - p);
                });
                *previous = Some(node.val);
                if node.right.is_some() {
                    result = min(result, Self::worker(&node.right, previous));
                }

                result
            }
            None => {
                i32::max_value()
            }
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut previous = None;
        Self::worker(&root, &mut previous)
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[4,2,6,1,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::get_minimum_difference(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let data = "[1,0,48,null,null,12,49]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::get_minimum_difference(root);
        assert_eq!(result, 1);
    }

}
