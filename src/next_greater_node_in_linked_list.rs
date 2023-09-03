use crate::list_node::ListNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    index: usize,
    value: i32,
}

impl Item {

    pub fn new(index: usize, value: i32) -> Self {
        Self { index, value }
    }

}

/// You are given the `head` of a linked list with `n` nodes.
///
/// For each node in the list, find the value of the next greater node. That is, for each node,
/// find the value of the first node that is next to it and has a strictly larger value than it.
///
/// Return an integer array `answer` where `answer[i]` is the value of the next greater node of the
/// `ith` node (1-indexed). If the `ith` node does not have a next greater node, set `answer[i] =
/// 0`.
struct Solution;

impl Solution {

    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut index = 0;
        let mut result = Vec::new();
        let mut stack: Vec<Item> = Vec::new();

        let mut current = head;
        while let Some(node) = current {
            let value = node.val;
            let item = Item::new(index, value);
            result.push(0);

            while !stack.is_empty() {
                let last = stack.last().copied().unwrap();
                if value > last.value {
                    stack.pop();
                    result[last.index] = value;
                } else {
                    break;
                }
            }
            stack.push(item);

            current = node.next;
            index += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![2,1,5];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::next_larger_nodes(head);
        assert_eq!(result, vec![5,5,0]);
    }

    #[test]
    fn example_2() {
        let items = vec![2,7,4,3,5];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::next_larger_nodes(head);
        assert_eq!(result, vec![7,0,5,5,0]);
    }

}
