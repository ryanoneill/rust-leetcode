use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use crate::vec_additions::VecAdditions;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Serialization is the process of converting a data structure or object into
/// a sequence of bits so that it can be stored in a file or memory buffer, or
/// transmitted across a network connection link to be reconstructed later in
/// the same or another computer environment.
///
/// Design an algorithm to serialize and deserialize a binary tree. There is no
/// restriction on how your serialization/deserialization algorithm should
/// work. You just need to ensure that a binary tree can be serialized to a
/// string and this string can be deserialized to the original tree structure.
///
/// Clarification: The input/output format is the same as how LeetCode
/// serializes a binary tree. You do not necessarily need to follow this format
/// so please be creative and come up with different approaches yourself.
pub struct Codec;

impl Codec {

    pub fn new() -> Self {
        Codec { }
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() { "[]".to_string() }
        else {
            let mut queue = VecDeque::new();
            let mut items = Vec::new();

            queue.push_back(root.clone());
            while !queue.is_empty() {
                let item = queue.pop_front().unwrap();
                let value = item.get_value();
                let value = value.map(|v| v.to_string()).unwrap_or("null".to_string());
                items.push(value);

                if item.is_some() {
                    let rc = item.unwrap();
                    let node = rc.borrow();
                    let left = node.left.clone();
                    let right = node.right.clone();
                    queue.push_back(left);
                    queue.push_back(right);
                }
            }
            // Remove unneeded "null"s at end
            items.pop_while(|i| i == "null");

            // Concatenate into comma separated list with brackets.
            let mut result = "[".to_string();
            result += &items.join(",");
            result += "]";

            result
        }

    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == "[]" { None }
        else {
            let items = data.strip_prefix("[")
                .and_then(|d| d.strip_suffix("]"))
                .map(|d| d.split(',').collect())
                .unwrap_or(vec![]);
            let items: Vec<Option<i32>> = items.iter()
                .map(|&v| v.parse().ok())
                .collect();
            let items: Vec<Option<Rc<RefCell<TreeNode>>>> = items.iter()
                .map(|v| {
                    match v.as_ref() {
                        Some(value) => TreeNodeAdditions::new(*value),
                        None => None
                    }
                })
                .collect();

            if items.is_empty() { None }
            else {
                let mut queue = VecDeque::new();
                let mut is_left = true;
                let mut current: Option<Rc<RefCell<TreeNode>>> = None;
                let result = items[0].clone();

                queue.push_back(result.clone());

                for index in 1..items.len() {
                    if is_left {
                        current = queue.pop_front().unwrap(); // within length of items
                        let left = &items[index];
                        current.set_left(left.clone());
                        if left.is_some() {
                            queue.push_back(left.clone());
                        }
                    } else {
                        let right = &items[index];
                        current.set_right(right.clone());
                        if right.is_some() {
                            queue.push_back(right.clone());
                        }
                    }
                    is_left = !is_left;
                }

                result
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::tree_node_additions::TreeNodeAdditions;
    use super::Codec;

    #[test]
    fn serialize_none() {
        let codec = Codec::new();
        let result = codec.serialize(None);
        assert_eq!(result, "[]");
    }

    #[test]
    fn serialize_single_node() {
        let codec = Codec::new();
        let one = TreeNodeAdditions::new(1);
        let result = codec.serialize(one);
        assert_eq!(result, "[1]");
    }

    #[test]
    fn deserialize_brackets() {
        let codec = Codec::new();
        let result = codec.deserialize("[]".to_string());
        assert_eq!(result, None);
    }

    #[test]
    fn deserialize_single_node() {
        let codec = Codec::new();
        let result = codec.deserialize("[1]".to_string());
        let one = TreeNodeAdditions::new(1);
        assert_eq!(result, one);
    }

    #[test]
    fn deserialize_three_node() {
        let codec = Codec::new();
        let result = codec.deserialize("[1,2,3]".to_string());
        let three = TreeNodeAdditions::new(3);
        let two = TreeNodeAdditions::new(2);
        let one = TreeNodeAdditions::with_children(1, two, three);
        assert_eq!(result, one);
    }

    #[test]
    fn real_example_1() {
        let data = "[3,9,20,null,null,15,7]".to_string();
        // let items = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let seven = TreeNodeAdditions::new(7);
        let fifteen = TreeNodeAdditions::new(15);
        let twenty = TreeNodeAdditions::with_children(20, fifteen, seven);
        let nine = TreeNodeAdditions::new(9);
        let three = TreeNodeAdditions::with_children(3, nine, twenty);

        let codec = Codec::new();
        let result = codec.deserialize(data);
        assert_eq!(result, three);
    }

    #[test]
    fn real_example_2() {
        let data = "[2,null,3,null,4,null,5,null,6]".to_string();
        // let items = vec![Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)];
        let six = TreeNodeAdditions::new(6);
        let five = TreeNodeAdditions::with_children(5, None, six);
        let four = TreeNodeAdditions::with_children(4, None, five);
        let three = TreeNodeAdditions::with_children(3, None, four);
        let two = TreeNodeAdditions::with_children(2, None, three);

        let codec = Codec::new();
        let result = codec.deserialize(data);
        assert_eq!(result, two);
    }

    #[test]
    fn real_example_3() {
        let data = "[5,4,8,11,null,13,4,7,2,null,null,null,1]".to_string();

        let one = TreeNodeAdditions::new(1);
        let four_right = TreeNodeAdditions::with_children(4, None, one);
        let thirteen = TreeNodeAdditions::new(13);
        let eight = TreeNodeAdditions::with_children(8, thirteen, four_right);

        let two = TreeNodeAdditions::new(2);
        let seven = TreeNodeAdditions::new(7);
        let eleven = TreeNodeAdditions::with_children(11, seven, two);
        let four_left = TreeNodeAdditions::with_children(4, eleven, None);

        let five = TreeNodeAdditions::with_children(5, four_left, eight);

        let codec = Codec::new();
        let result = codec.deserialize(data);
        assert_eq!(result, five);
    }

    #[test]
    fn real_example_4() {
        let data = "[1,2]".to_string();
        let two = TreeNodeAdditions::new(2);
        let one = TreeNodeAdditions::with_children(1, two, None);
        let codec = Codec::new();
        let result = codec.deserialize(data);
        assert_eq!(result, one);
    }

    #[test]
    fn real_example_5() {
        let data = "[1,-2,-3,1,3,-2,null,-1]".to_string();
        let neg_two_right = TreeNodeAdditions::new(-2);
        let neg_three = TreeNodeAdditions::with_children(-3, neg_two_right, None);

        let three = TreeNodeAdditions::new(3);
        let neg_one = TreeNodeAdditions::new(-1);
        let one_left = TreeNodeAdditions::with_children(1, neg_one, None);
        let neg_two_left = TreeNodeAdditions::with_children(-2, one_left, three);

        let one = TreeNodeAdditions::with_children(1, neg_two_left, neg_three);

        let codec = Codec::new();
        let result = codec.deserialize(data);
        assert_eq!(result, one);
    }



}

