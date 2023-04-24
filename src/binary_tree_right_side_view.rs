use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree, imagine yourself standing on the right
/// side of it, return the values of the nodes you can see ordered from top
/// to bottom.
struct Solution;

impl Solution {

    fn right_side_view_ref(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                result.push(node.val);
                Self::right_side_view_ref(&node.right, result)
            }
            None => {
                // do nothing
            }
        }
    }

    // TODO: Implement
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { vec![] }
        else {
            let mut result = Vec::new();
            Self::right_side_view_ref(&root, &mut result);

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[1,2,3,null,5,null,4]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1,3,4]);
    }

    #[test]
    fn example_2() {
        let data = "[1,null,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn example_3() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![]);
    }

}
