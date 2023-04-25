use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary search tree and a `target` value, return the
/// value in the BST that is closest to the `target`. If there are multiple
/// answers, print the smallest.
struct Solution;

impl Solution {

    fn worker(
        root: &Option<Rc<RefCell<TreeNode>>>,
        closest: &mut Option<i32>,
        target: f64
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let value = node.val as f64;
                match closest {
                    Some(c) => {
                        let c = *c as f64;
                        let old_diff = (c - target).abs();
                        let new_diff = (value - target).abs();
                        if new_diff < old_diff {
                            *closest = Some(node.val);
                        } else if new_diff == old_diff && value < c {
                            *closest = Some(node.val);
                        }
                    }
                    None => {
                        *closest = Some(node.val);
                    }
                }
                if value < target {
                    if node.right.is_some() {
                        Self::worker(&node.right, closest, target);
                    }
                } else if value > target {
                    if node.left.is_some() {
                        Self::worker(&node.left, closest, target);
                    }
                }
            }
            None => {
                // do nothing
            }
        }
    }

    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut closest: Option<i32> = None;
        Self::worker(&root, &mut closest, target);
        closest.unwrap_or_default()
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[4,2,5,1,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::closest_value(root, 3.714286);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let data = "[1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::closest_value(root, 4.428571);
        assert_eq!(result, 1);
    }

    #[test]
    fn equidistant() {
        let data = "[4,2,5,1,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::closest_value(root, 3.5);
        assert_eq!(result, 3);
    }

}
