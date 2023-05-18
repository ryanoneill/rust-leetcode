use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

/// You are given the `root` of a binary tree.
///
/// A ZigZag path for a binary tree is defined as follows:
///
/// * Choose any node in the binary tree and a direction (right or left).
///
/// * If the current direction is right, move to the right child of the current
///   node; otherwise, move to the left child.
///
/// * Change the direction from right to left or from left to right.
///
/// * Repeat the second and third steps until you can't move in the tree.
///
/// Zigzag length is defined as the number of nodes visited - 1. (A single node
/// has a length of 0).
///
/// Return the longest ZigZag path contained in that tree.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, count: i32, direction: Direction) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut direction = direction;
                if direction == Direction::Left {
                    direction = Direction::Right;
                } else {
                    direction = Direction::Left;
                }
                let mut result = count;
                let left_count;
                let right_count;

                if direction == Direction::Left {
                    left_count = count + 1;
                    right_count = 0;
                } else {
                    left_count = 0;
                    right_count = count + 1;
                }

                if node.left.is_some() {
                    result = result.max(Self::worker(&node.left, left_count, direction));
                }
                if node.right.is_some() {
                    result = result.max(Self::worker(&node.right, right_count, direction));
                }

                result
            }
            None => {
                0
            }
        }
    }

    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Self::worker(&root, 0, Direction::Right);
        result.max(Self::worker(&root, 0, Direction::Left))
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[1,null,1,1,1,null,null,1,1,null,1,null,null,null,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::longest_zig_zag(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let data = "[1,1,1,null,1,null,null,1,1,null,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::longest_zig_zag(root);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let data = "[1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::longest_zig_zag(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn real_world_1() {
        let data = "[1,null,2,3,4,null,null,null,5]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::longest_zig_zag(root);
        assert_eq!(result, 2);
    }

}
