use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::rc::Rc;
use std::cell::RefCell;

/// Given a binary tree, find the lowest common ancestor (LCA) of two given
/// nodes in the tree.
///
/// Accoring to the definition of LCA on Wikipedia: "The lowest common ancestor
/// is defined between two nodes `p` and `q` as the lowest node in `T` that has
/// both `p` and `q` as descendants (where we allow a node to be a descendant
/// of itself)."
///
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn lowest_common_ancestor(
        _root: Option<Rc<RefCell<TreeNode>>>,
        _p: Option<Rc<RefCell<TreeNode>>>,
        _q: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

}

#[cfg(test)]
mod tests {
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        // let items = vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)];
        let four: Option<Rc<RefCell<TreeNode>>> = TreeNodeAdditions::new(4);
        let seven = TreeNodeAdditions::new(7);
        let two = TreeNodeAdditions::with_children(2, seven, four);
        let six = TreeNodeAdditions::new(6);
        let five = TreeNodeAdditions::with_children(5, six, two);

        let five_clone: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>> = five.clone();

        let eight: Option<Rc<RefCell<TreeNode>>> = TreeNodeAdditions::new(8);
        let zero = TreeNodeAdditions::new(0);
        let one = TreeNodeAdditions::with_children(1, zero, eight);

        let one_clone = one.clone();

        let three = TreeNodeAdditions::with_children(3, five, one);
        let result = Solution::lowest_common_ancestor(three, five_clone, one_clone);
        assert_eq!(result.get_value(), Some(3));
    }

    // TODO: More tests

}
