use crate::list_node::ListNode;

/// Given the `head` of a singly linked list, reverse the list, and return the reversed list.
struct Solution;

impl Solution {

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous: Option<Box<ListNode>> = None;
        let mut current: Option<Box<ListNode>> = head;

        while let Some(current_node) = current.as_mut() {
            let next_node = current_node.next.take();
            current.as_mut().unwrap().next = previous;
            previous = current;
            current = next_node;
        }

        previous
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
        let result = Solution::reverse_list(nodes);
        assert_eq!(ListNode::to_vec(result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 2];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_list(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2, 1]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let nodes = ListNode::from_vec(items);
        let result = Solution::reverse_list(nodes);
        assert_eq!(ListNode::to_vec(result), vec![]);
    }

}
