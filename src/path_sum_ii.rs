use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Given the `root` of a binary tree and an integer `targetSum`, return all
/// root-to-leaf paths where the sum of the node values in the path equals
/// `targetSum`. Each path should be returned as a list of the node values,
/// not node references.
///
/// A root-to-leaf path is a path starting from the root and ending at any
/// leaf node. A leaf is a node with no children.
struct Solution;

impl Solution {

    fn path_sum_worker(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        path: Vec<i32>,
        results: &mut Vec<Vec<i32>>
    ) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let is_leaf = node.left.is_none() && node.right.is_none();
                let value = node.val;

                let mut path = path;
                path.push(value);
                if is_leaf {
                    if value == target_sum {
                        results.push(path);
                    }
                } else {
                    let new_target = target_sum - value;
                    if node.left.is_some() {
                        Self::path_sum_worker(node.left.clone(), new_target, path.clone(), results);
                    }
                    if node.right.is_some() {
                        Self::path_sum_worker(node.right.clone(), new_target, path.clone(), results);
                    }
                }
            }
            None => { }
        }

    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let path = vec![];
        let mut results = Vec::new();
        Self::path_sum_worker(root, target_sum, path, &mut results);
        results
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[5,4,8,11,null,13,4,7,2,null,null,5,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let mut result = Solution::path_sum(root, 22);
        result.sort();
        assert_eq!(result, vec![vec![5,4,11,2], vec![5,8,4,5]]);
    }

    #[test]
    fn example_2() {
        let data = "[1,2,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let mut result = Solution::path_sum(root, 5);
        result.sort();
        let empty_result: Vec<Vec<i32>> = vec![];
        assert_eq!(result, empty_result);
    }

    #[test]
    fn example_3() {
        let data = "[1,2]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let mut result = Solution::path_sum(root, 0);
        result.sort();
        let empty_result: Vec<Vec<i32>> = vec![];
        assert_eq!(result, empty_result);
    }


}
