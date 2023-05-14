use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a singly linked list, sort the list using insertion
/// sort, and return the sorted list's head.
///
/// The steps of the insertion sort algorithm:
///
/// Insertion sort iterates, consuming one input element each repetition and
/// growing a sorted output list.
///
/// At each iteration, insertion sort removes one element from the input data,
/// finds the location it belongs within the sorted list and inserts it there.
///
/// It repeats until no input elements remain.
struct Solution;

impl Solution {

    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut result = None;

        while head.is_some() {
            let mut new_node = head;
            head = new_node.take_next();

            if result.is_none() {
                result = new_node;
            } else if new_node.get_value() < result.get_value() {
                new_node.set_next(result);
                result = new_node;
            } else {
                let mut previous = &mut result;
                loop {
                    if previous.has_next() && (new_node.get_value() > previous.get_next_value()) {
                        previous = previous.refer_next();
                    } else { break; }
                }
                new_node.set_next(previous.take_next());
                previous.set_next(new_node);
            }
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
        let items = vec![4,2,1,3];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::insertion_sort_list(head);
        assert_eq!(result.to_vec(), vec![1,2,3,4]);
    }

    #[test]
    fn example_2() {
        let items = vec![-1,5,3,4,0];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::insertion_sort_list(head);
        assert_eq!(result.to_vec(), vec![-1,0,3,4,5]);
    }

}

