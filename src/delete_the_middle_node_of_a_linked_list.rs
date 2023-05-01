use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_empty() {
            None
        } else if !head.has_next() {
            None
        } else {
            let count = head.len();
            let to_find = count / 2 - 1;
            let before_middle = head.advance(to_find);
            let mut middle = before_middle.take_next();
            let continued = middle.take_next();
            before_middle.set_next(continued);

            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1, 3, 4, 7, 1, 2, 6];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(result.to_vec(), vec![1, 3, 4, 1, 2, 6]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 2, 3, 4];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(result.to_vec(), vec![1, 2, 4]);
    }

    #[test]
    fn example_3() {
        let items = vec![2, 1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(result.to_vec(), vec![2]);
    }

    #[test]
    fn empty() {
        let items = vec![];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_middle(nodes);
        assert_eq!(result.to_vec(), vec![]);
    }
}
