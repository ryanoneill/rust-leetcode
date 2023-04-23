use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a linked list, remove the `nth` node from the end of
/// the list and return its head.
pub struct Solution;

impl Solution {

    pub fn remove_nth_node_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut head = head;
        let len = head.len();
        if n >= len {
            head.take_next()
        } else {
            let steps = len - n - 1;
            let before = head.advance(steps);
            before.remove_next();

            head
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 2);
        assert_eq!(result.to_vec(), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example_2() {
        let items = vec![1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 1);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let items = vec![1, 2];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::remove_nth_node_from_end(nodes, 1);
        assert_eq!(result.to_vec(), vec![1]);
    }

}
