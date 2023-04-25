use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, imagine yourself standing on the right
/// side of it, return the values of the nodes you can see ordered from top
/// to bottom.
struct Solution;

impl Solution {

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { vec![] }
        else {
            let mut result = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(root.clone());

            while !queue.is_empty() {
                let len = queue.len();
                let right_side = queue.back().unwrap();
                right_side.get_value().map(|v| {
                    result.push(v);
                    v
                });

                for _ in 0..len {
                    let item = queue.pop_front().unwrap();

                    match item {
                        Some(rc) => {
                            let node = rc.borrow();
                            if node.left.is_some() {
                                queue.push_back(node.left.clone());
                            }
                            if node.right.is_some() {
                                queue.push_back(node.right.clone());
                            }
                        }
                        None => {
                            // do nothing
                        }
                    }
                }

            }

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

    #[test]
    fn example_4() {
        let data = "[1,2]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1,2]);
    }

}
