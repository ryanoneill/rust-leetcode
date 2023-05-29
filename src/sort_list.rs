use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the head of a linked list, return the list after sorting it in
/// ascending order.
struct Solution;

impl Solution {

    pub fn sort_list(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list.is_none() || !list.has_next() {
            list
        } else {
            let mut list1 = list;
            let len = list1.len();
            let mid = len / 2;
            let mut list2 = list1.split_at(mid);
            list1 = Self::sort_list(list1);
            list2 = Self::sort_list(list2);
            ListNodeAdditions::merge_lists(list1, list2)
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![4,2,1,3];
        let list = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_list(list);
        assert_eq!(result.to_vec(), vec![1,2,3,4]);
    }

    #[test]
    fn example_2() {
        let items = vec![-1,5,3,4,0];
        let list = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_list(list);
        assert_eq!(result.to_vec(), vec![-1,0,3,4,5]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let list = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_list(list);
        assert_eq!(result.to_vec(), vec![]);
    }

}
