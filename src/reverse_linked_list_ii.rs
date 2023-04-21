use crate::list_node::ListNode;

/// Given the `head` of a singly linked list and two integers `left` and
/// `right` where `left <= right`, reverse the nodes of the list from position
/// `left` to position `right`, and return the reversed list.
struct Solution;

impl Solution {

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;

        while current.is_some() {
            let next_node = Self::take_next(&mut current);
            Self::set_next(&mut current, previous);
            previous = current;
            current = next_node;
        }

        previous
    }

    fn advance(node: &mut Option<Box<ListNode>>, n: usize) -> &mut Option<Box<ListNode>> {
        let mut result = node;
        for _ in 1..=n {
            result = Self::refer_next(result);
        }
        result
    }

    fn refer_next(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        &mut node.as_mut().unwrap().next
    }

    fn has_next(node: &mut Option<Box<ListNode>>) -> bool {
        node.as_ref().unwrap().next.is_some()
    }

    fn take_next(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        node.as_mut().unwrap().next.take()
    }

    fn set_next(node: &mut Option<Box<ListNode>>, value: Option<Box<ListNode>>) {
        node.as_mut().unwrap().next = value
    }

    fn add_to_end(node: &mut Option<Box<ListNode>>, value: Option<Box<ListNode>>) {
        let mut current = node;
        while Self::has_next(current) {
            current = Self::refer_next(current);
        }
        Self::set_next(current, value);
    }

    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        if left >= right { head }
        else if left == 1 {
            let right_node = Self::advance(&mut head, right as usize - 1);
            let rest = Self::take_next(right_node);
            let mut result = Self::reverse_list(head);
            Self::add_to_end(&mut result, rest);
            result
        } else {
            let before_left = Self::advance(&mut head, left as usize - 2);
            let mut start = Self::take_next(before_left);
            let right_node = Self::advance(&mut start, (right - left) as usize);
            let rest = Self::take_next(right_node);
            let mut reversed = Self::reverse_list(start);
            Self::add_to_end(&mut reversed, rest);
            Self::add_to_end(&mut head, reversed);

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
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_between(nodes, 2, 4);
        assert_eq!(ListNode::to_vec(result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn example_2() {
        let items = vec![5];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_between(nodes, 1, 1);
        assert_eq!(ListNode::to_vec(result), vec![5]);
    }

    #[test]
    fn reverse_beginning() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_between(nodes, 1, 5);
        assert_eq!(ListNode::to_vec(result), vec![6, 7, 8, 9, 10, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_end() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_between(nodes, 3, 10);
        assert_eq!(ListNode::to_vec(result), vec![10, 9, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn reverse_middle() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_between(nodes, 4, 9);
        assert_eq!(ListNode::to_vec(result), vec![10, 9, 8, 2, 3, 4, 5, 6, 7, 1]);
    }

}
