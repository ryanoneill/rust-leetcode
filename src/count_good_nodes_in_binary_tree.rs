use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Given a binary tree `root`, a node X in the tree is named good if in the
/// path from root to X there are no nodes with a value greater than X.
///
/// Return the number of good nodes in the binary tree.
struct Solution;

impl Solution {
    fn count_good_nodes(root: &Option<Rc<RefCell<TreeNode>>>, max_seen: i32) -> usize {
        match root {
            None => 0,
            Some(rc) => {
                let node = rc.borrow();
                let mut result = 0;
                let mut new_max = max_seen;

                if node.val >= max_seen {
                    new_max = node.val;
                    result += 1;
                }

                result += Self::count_good_nodes(&node.left, new_max);
                result += Self::count_good_nodes(&node.right, new_max);

                result
            }
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_good_nodes(&root, i32::min_value()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;

    #[test]
    fn example_1() {
        let data = "[3,1,4,3,null,1,5]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::good_nodes(root);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let data = "[3,3,null,4,2]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::good_nodes(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let data = "[1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::good_nodes(root);
        assert_eq!(result, 1);
    }
}
