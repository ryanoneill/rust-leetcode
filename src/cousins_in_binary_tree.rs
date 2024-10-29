use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Given the `root` of a binary tree with unique values and the values of the two different nods
/// of the tree `x` and `y`, return `true` if the nodes corresponding to the values `x` and `y` in
/// the tree are cousins, or `false` otherwise.
///
/// Two nodes of a binary tree are cousins if they have the same depth with different parents.
///
/// Note that in a binary tree, the root node is at depth `0`, and children of reach depth `k` node
/// are at the depth `k + 1`.
struct Solution;

impl Solution {

    fn worker(
        info: &mut HashMap<i32, (i32, usize)>,
        parent: i32,
        depth: usize,
        root: &Option<Rc<RefCell<TreeNode>>>
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let val = node.val;
                info.insert(val, (parent, depth));
                if node.left.is_some() {
                    Self::worker(info, val, depth + 1, &node.left);
                }
                if node.right.is_some() {
                    Self::worker(info, val, depth + 1, &node.right);
                }
            }
            None => { }
        }
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut info = HashMap::new();
        Self::worker(&mut info, 0, 0, &root);

        let info_x = info[&x];
        let info_y = info[&y];

        // Parents aren't equal and depths are
        (info_x.0 != info_y.0) && (info_x.1 == info_y.1)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,2,3,4]");
        let x = 4;
        let y = 3;
        let result = Solution::is_cousins(root, x, y);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2,3,null,4,null,5]");
        let x = 5;
        let y = 4;
        let result = Solution::is_cousins(root, x, y);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let root = tree!("[1,2,3,null,4]");
        let x = 2;
        let y = 3;
        let result = Solution::is_cousins(root, x, y);
        assert!(!result);
    }

}
