use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// You are given the heads of two sorted linked lists `list1` and `list2`.
///
/// Merge the two lists in a one sorted list. The list should be made by
/// splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
pub struct Solution;

impl Solution {

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        ListNodeAdditions::merge_lists(list1, list2)
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items1 = vec![1, 2, 4];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![1, 3, 4];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example_2() {
        let items1 = vec![];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let items1 = vec![];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![0];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![0]);
    }
}
