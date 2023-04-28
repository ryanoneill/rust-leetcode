use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a sorted linked list, delete all duplicates such that
/// each element appears only once. Return the linked list sorted as well.
struct Solution;

impl Solution {

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        if head != None {
            let mut previous = &mut head;
            let mut last_value = previous.get_value();
            let mut current = previous.take_next();

            while !current.is_empty() {
                if current.get_value() != last_value {
                    last_value = current.get_value();
                    previous.set_next(current);
                    previous = previous.refer_next();
                    current = previous.take_next();
                } else {
                    current = current.take_next();
                }
            }

        }

        head
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let head = ListNodeAdditions::from_vec(vec![1, 1, 2]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(result.to_vec(), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let head = ListNodeAdditions::from_vec(vec![1, 1, 2, 3, 3]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(result.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn all_ones() {
        let head = ListNodeAdditions::from_vec(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(result.to_vec(), vec![1]);
    }

    #[test]
    fn one_one_two_two() {
        let head = ListNodeAdditions::from_vec(vec![1, 1, 2, 2, 1, 1, 2, 2]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(result.to_vec(), vec![1, 2, 1, 2]);
    }

}
