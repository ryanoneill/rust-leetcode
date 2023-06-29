use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the level order traversal of its nodes' values.
/// (i.e., from left to right, level by level).
struct Solution;

impl Solution {

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let n = queue.len();
            let mut level = Vec::new();
            for _ in 0..n {
                let item = queue.pop_front().unwrap();
                match item {
                    Some(rc) => {
                        let node = rc.borrow();
                        level.push(node.val);
                        queue.push_back(node.left.clone());
                        queue.push_back(node.right.clone());
                    }
                    None => { } // do nothing
                }
            }
            if !level.is_empty() {
                results.push(level);
            }
        }

        results
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[3,9,20,null,null,15,7]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::level_order(root);
        assert_eq!(result, vec![vec![3], vec![9,20], vec![15,7]]);
    }

    #[test]
    fn example_2() {
        let data = "[1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn example_3() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::level_order(root);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

}
