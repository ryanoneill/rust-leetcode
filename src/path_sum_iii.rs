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

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, path: Vec<i64>) -> i32 {
        match root {
            Some(rc) => {
                let mut path = path;
                let node = rc.borrow();
                let value = node.val;
                let mut result = 0;

                path = path.iter_mut().map(|v| *v + value as i64).collect();
                path.push(value as i64);
                for sum in &path {
                    if *sum == target_sum as i64 {
                        result += 1;
                    }
                }

                if node.left.is_some() {
                    result += Self::worker(&node.left, target_sum, path.clone());
                }
                if node.right.is_some() {
                    result += Self::worker(&node.right, target_sum, path.clone());
                }
                result
            }
            None => {
                0
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::worker(&root, target_sum, Vec::new())
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[10,5,-3,3,2,null,11,3,-2,null,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 8);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let data = "[5,4,8,11,null,13,4,7,2,null,null,5,1]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 22);
        assert_eq!(result, 3);
    }

    #[test]
    fn empty() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn large_numbers() {
        let data = "[1000000000,1000000000,null,294967296,null,1000000000,null,1000000000,null,1000000000]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::path_sum(root, 0);
        assert_eq!(result, 0);
    }

}
