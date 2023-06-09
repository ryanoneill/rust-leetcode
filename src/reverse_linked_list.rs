use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a singly linked list, reverse the list, and return the reversed list.
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_list(nodes);
        assert_eq!(result.to_vec(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 2];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_list(nodes);
        assert_eq!(result.to_vec(), vec![2, 1]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_list(nodes);
        assert_eq!(result.to_vec(), vec![]);
    }
}
