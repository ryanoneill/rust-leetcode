use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a singly linked list, return the middle node of the
/// linked list.
///
/// If there are two middle nodes, return the second middle node.
struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slower = &head;
        let mut faster = head.get_next();

        while faster.is_some() {
            slower = slower.get_next();
            faster = faster.get_next();
            if faster.is_some() {
                faster = faster.get_next();
            }
        }

        slower.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4, 5];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::middle_node(head);
        assert_eq!(result.to_vec(), vec![3, 4, 5]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::middle_node(head);
        assert_eq!(result.to_vec(), vec![4, 5, 6]);
    }
}
