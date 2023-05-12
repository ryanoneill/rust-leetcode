use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a singly linked list, group the nodes with odd indices
/// together followed by the nodes with even indices, and return the reordered
/// list.
///
/// The first node is considered odd, and the second node is even, and so on.
///
/// Note that the relative order inside both the even and odd groups should
/// remain as it was in the input.
///
/// You must solve the problem in `O(1)` extra space complexity and `O(n)` time
/// complexity.
struct Solution;

impl Solution {

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        let mut is_odd_turn = true;
        let mut odd_head: Option<Box<ListNode>> = None;
        let mut odd = &mut odd_head;
        let mut even_head: Option<Box<ListNode>> = None;
        let mut even = &mut even_head;

        while head.is_some() {
            let mut node = head;
            head = node.take_next();

            if is_odd_turn {
                if odd.is_none() {
                    odd_head = node;
                    odd = &mut odd_head;
                } else {
                    odd.set_next(node);
                    odd = odd.refer_next();
                }
            } else {
                if even.is_none() {
                    even_head = node;
                    even = &mut even_head;
                } else {
                    even.set_next(node);
                    even = even.refer_next();
                }
            }
            is_odd_turn = !is_odd_turn;
        }

        if even_head.is_some() {
            odd.set_next(even_head);
        }
        odd_head
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![1,2,3,4,5];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::odd_even_list(head);
        assert_eq!(result.to_vec(), vec![1,3,5,2,4]);
    }

    #[test]
    fn example_2() {
        let items = vec![2,1,3,5,6,4,7];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::odd_even_list(head);
        assert_eq!(result.to_vec(), vec![2,3,6,7,1,5,4]);
    }

}
