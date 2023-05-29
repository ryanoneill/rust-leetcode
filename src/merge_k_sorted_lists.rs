use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;
use std::collections::VecDeque;

/// You are given an array of `k` linked-lists `lists`, each linked-list is
/// sorted in ascending order.
///
/// Merge all the linked-lists into one sorted linked-list and return it.
struct Solution;

impl Solution {

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue: VecDeque<Option<Box<ListNode>>> = lists.into();
        while queue.len() > 1 {
            let list1 = queue.pop_front().unwrap();
            let list2 = queue.pop_front().unwrap();
            let merged = ListNodeAdditions::merge_lists(list1, list2);
            queue.push_back(merged);
        }
        queue.pop_front().unwrap_or(None)
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let data = vec![vec![1,4,5], vec![1,3,4], vec![2,6]];
        let lists = data.into_iter().map(ListNodeAdditions::from_vec).collect();
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result.to_vec(), vec![1,1,2,3,4,4,5,6]);
    }

    #[test]
    fn example_2() {
        let lists = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let lists = vec![None];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result.to_vec(), vec![]);
    }

}
