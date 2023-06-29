use crate::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

/// Given the `root` of a binary search tree, and an integer `k`, return the `kth` smallest value
/// (1-indexed) of all the values of the nodes in the tree.
struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, result: &mut Option<i32>) {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                Self::worker(&node.left, k, count, result);
                *count += 1;
                if *count < k {
                    Self::worker(&node.right, k, count, result);
                } else if *count == k {
                    *result = Some(node.val);
                }
            }
            None => { } // do nothing
        }

    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count: i32 = 0;
        let mut result: Option<i32> = None;
        Self::worker(&root, k, &mut count, &mut result);
        result.unwrap_or_default()
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize_and_deserialize_binary_tree::Codec;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = "[3,1,4,null,2]".to_string();
        let k = 1;
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::kth_smallest(root, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let data = "[5,3,6,2,4,null,null,1]".to_string();
        let k = 3;
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::kth_smallest(root, k);
        assert_eq!(result, 3);
    }

}
