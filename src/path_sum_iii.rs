use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree and an integer `targetSum`, return the
/// number of paths where the sum of the values along the path equals
/// `targetSum`.
///
/// The path does not need to start or end at the root or a leaf, but it must
/// go downwards (i.e., traveling only from parent nodes to child nodes).
struct Solution;

impl Solution {

    // TODO: Implement
    pub fn path_sum(_root: Option<Rc<RefCell<TreeNode>>>, _target_sum: i32) -> i32 {
        0
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let data = "[10,5,-3,3,2,null,11,3,-2,null,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 8);
        assert_eq!(result, 3);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let data = "[5,4,8,11,null,13,4,7,2,null,null,5,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 22);
        assert_eq!(result, 3);
    }

    #[ignore]
    #[test]
    fn empty() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 0);
        assert_eq!(result, 0);
    }

}
