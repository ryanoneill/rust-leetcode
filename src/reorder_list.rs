use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// You are given the head of a singly linked-list. The list can be represented as:
///
/// `L0 -> L1 -> ... -> Ln-1 -> Ln`
///
/// Reorder the list to be on the following form:
///
/// `L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...`
///
/// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
struct Solution;

impl Solution {

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_some() {
            let len = head.len();
            let half = if len % 2 == 1 {
                len / 2 + 1
            } else {
                len / 2
            };

            let mut back = head.split_at(half).reverse();
            let mut list1 = head.take_next();
            let mut list2 = if back.is_none() {
                None
            } else {
                back.take_next()
            };

            head.set_next(back);
            let mut current = head.refer_next();
            let mut list1_turn = true;

            while list1.is_some() || list2.is_some() {
                if list1.is_some() && list1_turn {
                    let next = list1.take_next();
                    current.set_next(list1);
                    current = current.refer_next();
                    list1 = next;
                    list1_turn = false;
                } else {
                    let next = list2.take_next();
                    current.set_next(list2);
                    current = current.refer_next();
                    list2 = next;
                    list1_turn = true;
                }
            }

        }

    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1,2,3,4];
        let mut head = ListNodeAdditions::from_vec(items);
        Solution::reorder_list(&mut head);
        let result = head.to_vec();
        assert_eq!(result, vec![1,4,2,3]);
    }

    #[test]
    fn example_2() {
        let items = vec![1,2,3,4,5];
        let mut head = ListNodeAdditions::from_vec(items);
        Solution::reorder_list(&mut head);
        let result = head.to_vec();
        assert_eq!(result, vec![1,5,2,4,3]);
    }

    #[test]
    fn example_empty() {
        let items = vec![];
        let mut head = ListNodeAdditions::from_vec(items);
        Solution::reorder_list(&mut head);
        let result = head.to_vec();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_single() {
        let items = vec![1];
        let mut head = ListNodeAdditions::from_vec(items);
        Solution::reorder_list(&mut head);
        let result = head.to_vec();
        assert_eq!(result, vec![1]);
    }

}
