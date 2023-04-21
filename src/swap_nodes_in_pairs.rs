use crate::list_node::ListNode;

/// Given a linked list, swap every two adjacent nodes and return its head.
/// You must solve the problem without modifying the values in the list's nodes
/// (i.e., only nodes themselves may be changed.)
struct Solution;

impl Solution {

    fn set_next(node: &mut Option<Box<ListNode>>, value: Option<Box<ListNode>>) {
        node.as_mut().unwrap().next = value
    }

    fn take_next(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        node.as_mut().unwrap().next.take()
    }

    fn refer_next(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        &mut node.as_mut().unwrap().next
    }

    fn refer_next_next(node: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        &mut node.as_mut().unwrap().next.as_mut().unwrap().next
    }

    fn has_pair(node: &Option<Box<ListNode>>) -> bool {
        node.as_ref().is_some() && node.as_ref().unwrap().next.is_some()
    }

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut previous: &mut Option<Box<ListNode>> = &mut None;

        if current.as_ref().is_none() { None }
        else if current.as_ref().unwrap().next.is_none() { current }
        else {
            let mut result = None;
            let mut first = true;

            while Self::has_pair(&current) {
                if first {
                    // current is head, and taking head.next here
                    result = Self::take_next(&mut current); // 2
                    let former_head = current; // 1
                    current = Self::take_next(&mut result); // 3
                    Self::set_next(&mut result, former_head); // 2 -> 1
                    previous = Self::refer_next(&mut result); // 1
                    first = false;
                } else {
                    // current is 3
                    let mut next = Self::take_next(&mut current); // 4
                    let former_current = current; // 3
                    current = Self::take_next(&mut next); // 5
                    Self::set_next(&mut next, former_current); // 4 -> 3
                    Self::set_next(previous, next); // 1 -> 4
                    previous = Self::refer_next_next(previous);
                }
            }
            // if last value has no pair
            if current.is_some() {
                Self::set_next(previous, current);
            }

            result
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4];
        let nodes = ListNode::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn example_2() {
        let items = vec![];
        let nodes = ListNode::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(ListNode::to_vec(result), vec![]);
    }

    #[test]
    fn example_3() {
        let items = vec![1];
        let nodes = ListNode::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(ListNode::to_vec(result), vec![1]);
    }

    #[test]
    fn original_example() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let nodes = ListNode::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2, 1, 4, 3, 6, 5]);
    }

    #[test]
    fn odd_example() {
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNode::from_vec(items);
        let result = Solution::swap_pairs(nodes);
        assert_eq!(ListNode::to_vec(result), vec![2, 1, 4, 3, 5]);
    }

}
