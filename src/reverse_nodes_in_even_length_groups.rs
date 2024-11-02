use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// You are given the `head` of a linked list.
///
/// The nodes in the linked list are sequentially assigned to non-empty groups whose lenths form
/// the sequence of the natural numbers (`1, 2, 3, 4, ...`). The length of a group is the number of
/// nodes assigned to it. In other words,
///
/// * The `1st` node is assigned to the first group.
///
/// * The `2nd` and the `3rd` nodes are assigned to the second group.
///
/// * The `4th`, `5th`, and `6th` nodes are assigned to the third group, and so on.
///
/// Note that the length of the last group may be less than or equal to `1 + the length of the
/// second to last group.`
///
/// Reverse the nodes in each group with an even length, and return the `head` of the modified
/// linked list.
struct Solution;

impl Solution {

    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n = head.len();
        let mut remain = n;

        let mut head = head;
        let mut result = None;
        let mut current = &mut None;

        let mut round = 1;

        while remain > 0 {
            let count = if remain >= round {
                round
            } else {
                remain
            };
            if round == 1 {
                let rest = head.split_at(round);
                result = head;
                current = &mut result;
                remain -= round;
                head = rest;
            } else if count % 2 == 0 {
                let rest = head.split_at(count);
                current.set_next(head.reverse());
                current = current.advance(count);
                remain -= count;
                head = rest;
            } else {
                let rest = head.split_at(count);
                current.set_next(head);
                current = current.advance(count);
                remain -= count;
                head = rest;
            }
            round += 1;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![5,2,6,3,9,1,7,3,8,4];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_even_length_groups(head);
        assert_eq!(result.to_vec(), vec![5,6,2,3,9,1,4,8,3,7]);
    }

    #[test]
    fn example_2() {
        let items = vec![1,1,0,6];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_even_length_groups(head);
        assert_eq!(result.to_vec(), vec![1,0,1,6]);
    }

    #[test]
    fn example_3() {
        let items = vec![1,1,0,6,5];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_even_length_groups(head);
        assert_eq!(result.to_vec(), vec![1,0,1,5,6]);
    }

}
