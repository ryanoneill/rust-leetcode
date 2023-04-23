use crate::{list_node::ListNode, list_node_additions::ListNodeAdditions};

/// Given a linked list, swap every two adjacent nodes and return its head.
/// You must solve the problem without modifying the values in the list's nodes
/// (i.e., only nodes themselves may be changed.)
struct Solution;

impl Solution {

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut previous: &mut Option<Box<ListNode>> = &mut None;

        if current.as_ref().is_none() { None }
        else if current.as_ref().unwrap().next.is_none() { current }
        else {
            let mut result = None;
            let mut first = true;

            while current.has_pair() {
                if first {
                    // current is head, and taking head.next here
                    result = current.take_next(); // 2
                    let former_head = current; // 1
                    current = result.take_next(); // 3
                    result.set_next(former_head); // 2 -> 3
                    previous = result.refer_next(); // 1
                    first = false;
                } else {
                    // current is 3
                    let mut next = current.take_next(); // 4
                    let former_current = current; // 3
                    current = next.take_next(); // 5
                    next.set_next(former_current); // 4 -> 3
                    previous.set_next(next); // 1 -> 4
                    previous = previous.refer_next_next();
                }
            }
            // if last value has no pair
            if current.is_some() {
                previous.set_next(current);
            }

            result
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(result.to_vec(), vec![2, 1, 4, 3]);
    }

    #[test]
    fn example_2() {
        let items = vec![];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let items = vec![1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(result.to_vec(), vec![1]);
    }

    #[test]
    fn original_example() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(result.to_vec(), vec![2, 1, 4, 3, 6, 5]);
    }

    #[test]
    fn odd_example() {
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(result.to_vec(), vec![2, 1, 4, 3, 5]);
    }

}
