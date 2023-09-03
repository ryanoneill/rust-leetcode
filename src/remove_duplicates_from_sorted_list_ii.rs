use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a sorted linked list, delete all nodes that have
/// duplicate numbers, leaving only distinct numbers from the original list.
/// Return the linked list sorted as well.
pub struct Solution;

impl Solution {

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = -101;
        let mut result = None;
        let mut current = &mut result;
        let mut is_set = false;

        let mut working = head;
        while working.is_some() {
            let next = working.take_next();
            let value = working.get_value();
            if value != last {
                last = value;

                if next.is_none() || next.get_value() != value {
                    if !is_set {
                        result = working;
                        current = &mut result;
                        is_set = true;
                    } else {
                        current.set_next(working);
                        current = current.refer_next();
                    }
                }
            }
            working = next;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 3, 4, 4, 5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_duplicates(nodes);
        assert_eq!(result.to_vec(), vec![1, 2, 5]);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 1, 1, 2, 3];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::delete_duplicates(nodes);
        assert_eq!(result.to_vec(), vec![2, 3]);
    }
}
