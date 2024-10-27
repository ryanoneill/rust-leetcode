use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// You are given the `root` of a binary tree containing digits from `0` to `9` only.
///
/// Each root-to-leaf path in the tree represents a number.
///
/// * For example, the root-to-leaf path `1 -> 2 -> 3` represents the number `123`.
///
/// Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer
/// will fit in a 32-bit integer.
///
/// A leaf node is a node with no children.
struct Solution;

impl Solution {

    fn worker(result: &mut i32, current: i32, root: &Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut current = current;
                current *= 10;
                current += node.val;
                if node.left.is_none() && node.right.is_none() {
                    *result += current;
                } else {
                    if node.left.is_some() {
                        Self::worker(result, current, &node.left);
                    }
                    if node.right.is_some() {
                        Self::worker(result, current, &node.right);
                    }
                }
            }
            None => { }
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::worker(&mut result, 0, &root);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3]");
        let result = Solution::sum_numbers(root);
        assert_eq!(result, 25);
    }

    #[test]
    fn example_2() {
        let root = tree!("[4,9,0,5,1]");
        let result = Solution::sum_numbers(root);
        assert_eq!(result, 1026);
    }

}
