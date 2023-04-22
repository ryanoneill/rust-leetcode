use crate::list_node::ListNode;

/// Given the `head` of a linked list, remove the `nth` node from the end of
/// the list and return its head.
pub struct Solution;

impl Solution {

    // TODO: Implement
    pub fn remove_nth_node_from_end(_head: Option<Box<ListNode>>, _n: i32) -> Option<Box<ListNode>> {
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
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNode::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 2);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 5]);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let items = vec![1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 1);
        assert_eq!(ListNode::to_vec(result), vec![]);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let items = vec![1, 2];
        let nodes = ListNode::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 1);
        assert_eq!(ListNode::to_vec(result), vec![1]);
    }

}
