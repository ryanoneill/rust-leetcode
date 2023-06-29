use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {

    fn worker(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let mut result = true;
                result = result && Self::worker(&node.left);
                result = result && Self::worker(&node.right);
                if result {
                    let left = Self::depth(&node.left);
                    let right = Self::depth(&node.right);
                    result = (left as i32 - right as i32).abs() <= 1;
                }
                result
            }
            None => true,
        }
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            None => 0,
            Some(rc) => {
                let node = rc.borrow();
                let left = Self::depth(&node.left);
                let right = Self::depth(&node.right);

                1 + left.max(right)
            }
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::worker(&root)
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
        let result = Solution::is_balanced(root);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let data = "[1,2,2,3,3,null,null,4,4]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::is_balanced(root);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let data = "[]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::is_balanced(root);
        assert!(result);
    }

}
