use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// In a linked list of size `n`, where `n` is even, the `ith` node (0-indexed)
/// of the linked list is known as the twin of the `(n-1-i)th` node, if
/// `0 <= i <= (n/2) - 1`.
///
/// * For example, if `n = 4`, then node `0` is the twin of node `3`, and node
///   `1` is the twin of node `2`. These are the only nodes with twins for
///   `n = 4`.
///
/// The twin sum is defined as the sum of a node and its twin.
///
/// Given the head of a linked list with even length, return the maximum twin
/// sum of the linked list.
struct Solution;

impl Solution {

    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let n = head.len();
        let mid = n / 2;
        let mut tail = head.split_at(mid);
        tail = tail.reverse();

        let mut head_current = &mut head;
        let mut tail_current = &mut tail;

        let mut max = 0;
        while head_current.is_some() && tail_current.is_some() {
            let head_value = head_current.get_value();
            let tail_value = tail_current.get_value();
            let sum = head_value + tail_value;
            max = max.max(sum);

            head_current = head_current.refer_next();
            tail_current = tail_current.refer_next();
        }

        max
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![5,4,2,1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::pair_sum(head);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let items = vec![4,2,2,3];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::pair_sum(head);
        assert_eq!(result, 7);
    }

}
