use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

impl Solution {

    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { vec![] }
        else {
            let mut result = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(root.clone());

            while !queue.is_empty() {
                let len = queue.len();
                let mut largest = i32::min_value();

                for _ in 0..len {
                    let item = queue.pop_front().unwrap();
                    match item {
                        Some(rc) => {
                            let node = rc.borrow();
                            let value = node.val;
                            largest = max(value, largest);
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
                result.push(largest);
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
        let data = "[1,3,2,5,3,null,9]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::largest_values(root);
        assert_eq!(result, vec![1,3,9]);
    }

    #[test]
    fn example_2() {
        let data = "[1,2,3]".to_string();
        let codec = Codec::new();
        let root = codec.deserialize(data);
        let result = Solution::largest_values(root);
        assert_eq!(result, vec![1,3]);
    }

}
