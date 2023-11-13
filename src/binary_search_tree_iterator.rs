use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Implement the `BSTIterator` class that represents an iterator over the in-order traversal of a
/// binary search tree (BST):
///
/// * `BSTIterator(TreeNode root)` Initializes an object of the `BSTIterator` class. The `root` of
///   the BST is given as part of the constructor. The pointer should be initialized to a
///   non-existent number smaller than any element in the BST.
///
/// * `boolean hasNext()` Returns `true` if there exists a number in the traversal to the right of
///   the pointer, otherwise returns `false`.
///
/// * `int next()` Moves the pointer to the right, then returns the number at the pointer.
///
/// Notice that by initializing the pointer to a non-existent smallest number, the first call to
/// `next()` will return the smallest element in the BST.
///
/// You may assume that `next()` calls will always be valid. That is, there will be at least a next
/// number in the in-order traversal when `next()` is called.
struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut current = root.clone();
        while current.is_some() {
            let next = current.clone_left();
            stack.push(current);
            current = next;
        }

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        let mut result = None;
        if self.has_next() {
            result = self.stack.pop().unwrap();

            let mut current = result.clone_right();
            while current.is_some() {
                let next = current.clone_left();
                self.stack.push(current);
                current = next;
            }
        }
        result.get_value().unwrap_or(-1)
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::BSTIterator;

    #[test]
    fn example_1() {
        let root = tree!("[7,3,15,null,null,9,20]");
        let mut bst_iterator = BSTIterator::new(root);
        let result = bst_iterator.next();
        assert_eq!(result, 3);
        let result = bst_iterator.next();
        assert_eq!(result, 7);
        let result = bst_iterator.has_next();
        assert!(result);
        let result = bst_iterator.next();
        assert_eq!(result, 9);
        let result = bst_iterator.has_next();
        assert!(result);
        let result = bst_iterator.next();
        assert_eq!(result, 15);
        let result = bst_iterator.has_next();
        assert!(result);
        let result = bst_iterator.next();
        assert_eq!(result, 20);
        let result = bst_iterator.has_next();
        assert!(!result);
    }
}
