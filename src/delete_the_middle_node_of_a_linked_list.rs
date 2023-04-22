use crate::list_node::ListNode;

/// You are given the head of a linked list. Delete the middle node, and return
/// the `head` of the modified linked list.
///
/// The middle node of a linked list of size `n` is the `⌊n / 2⌋th`th node from
/// the start using 0-based indexing, where `⌊x⌋` denotes the largest integer
/// less than or equal to `x`.
///
/// * For `n` = `1`, `2`, `3`, `4`, `5`, the middle nodes are `0`, `1`, `1`,
/// `2`, and `2`, respectively.
pub struct Solution;

impl Solution {

    fn has_next(node: &Option<Box<ListNode>>) -> bool {
        node.as_ref().unwrap().next.is_some()
    }

    fn refer_next(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        &mut node.as_mut().unwrap().next
    }

    fn refer_next_next(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        &mut node.as_mut().unwrap().next.as_mut().unwrap().next
    }

    fn take_next(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        node.as_mut().unwrap().next.take()
    }

    fn set_next(node: &mut Option<Box<ListNode>>, value: Option<Box<ListNode>>) {
        node.as_mut().unwrap().next = value
    }

    fn count(node: &Option<Box<ListNode>>) -> usize {
        let mut result = 0;
        let mut current = node;

        while current.as_ref().is_some() {
            result += 1;
            current = &current.as_ref().unwrap().next;
        }

        result
    }

    fn find_node(node: &mut Option<Box<ListNode>>, n: usize) -> &mut Option<Box<ListNode>> {
        let mut result = node;

        for _ in 1..=n {
            result = Self::refer_next(result);
        }

        result
    }

    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.as_ref().is_none() { None }
        else if !Self::has_next(&head) { None }
        else {
            let count = Self::count(&head);
            let to_find = count / 2 - 1;
            let before_middle = Self::find_node(&mut head, to_find);
            let mut middle = Self::take_next(before_middle);
            let continued = Self::take_next(&mut middle);
            Self::set_next(before_middle, continued);

            head
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1, 3, 4, 7, 1, 2, 6];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(ListNode::to_vec(result), vec![1, 3, 4, 1, 2, 6]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 2, 3, 4];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 4]);
    }

    #[test]
    fn example_3() {
        let items = vec![2, 1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2]);
    }

    #[test]
    fn empty() {
        let items = vec![];
        let nodes = ListNode::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(ListNode::to_vec(result), vec![]);
    }

}
