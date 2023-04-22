use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a sorted linked list, delete all nodes that have
/// duplicate numbers, leaving only distinct numbers from the original list.
/// Return the linked list sorted as well.
pub struct Solution;

impl Solution {

    // TODO: Implement
    pub fn delete_duplicates(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use super::Solution;

    #[ignore]
    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 3, 4, 4, 5];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_duplicates(nodes);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 5]);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let items = vec![1, 1, 1, 2, 3];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_duplicates(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2, 3]);
    }

}
