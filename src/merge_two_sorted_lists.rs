use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

pub struct Solution;

impl Solution {

    // TODO: Implement
    pub fn merge_two_lists(
        _list1: Option<Box<ListNode>>,
        _list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
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
        let items1 = vec![1, 2, 4];
        let list1 = ListNode::from_vec(items1);
        let items2 = vec![1, 3, 4];
        let list2 = ListNode::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(ListNode::to_vec(result), vec![1,1,2,3,4,4]);
    }

    #[ignore]
    #[test]
    fn example_2() {
        let items1 = vec![];
        let list1 = ListNode::from_vec(items1);
        let items2 = vec![];
        let list2 = ListNode::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(ListNode::to_vec(result), vec![]);
    }

    #[ignore]
    #[test]
    fn example_3() {
        let items1 = vec![];
        let list1 = ListNode::from_vec(items1);
        let items2 = vec![0];
        let list2 = ListNode::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(ListNode::to_vec(result), vec![0]);
    }

}
