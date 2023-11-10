use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a linked list and an integer `val`, remove all the nodes of the linked list
/// that has `Node.val == val`, and return the new head.
struct Solution;

impl Solution {

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result = None;

        let mut previous = &mut None;
        let mut current = head;

        while current.is_some() {
            let next = current.take_next();
            if current.get_value() != val {
                if previous.is_none() {
                    result = current;
                    previous = &mut result;
                } else {
                    previous.set_next(current);
                    previous = previous.refer_next();
                }
            }
            current = next;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1,2,6,3,4,5,6];
        let head = ListNodeAdditions::from_vec(items);
        let val = 6;
        let result = Solution::remove_elements(head, val);
        assert_eq!(result.to_vec(), vec![1,2,3,4,5]);
    }

    #[test]
    fn example_2() {
        let items = vec![];
        let head = ListNodeAdditions::from_vec(items);
        let val = 1;
        let result = Solution::remove_elements(head, val);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let head = ListNodeAdditions::from_vec(items);
        let val = 7;
        let result = Solution::remove_elements(head, val);
        assert_eq!(result.to_vec(), vec![]);
    }

}
