use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a singly linked list and two integers `left` and
/// `right` where `left <= right`, reverse the nodes of the list from position
/// `left` to position `right`, and return the reversed list.
struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        if left >= right {
            head
        } else if left == 1 {
            let right_node = head.advance(right as usize - 1);
            let rest = right_node.take_next();
            let mut result = head.reverse();
            result.add_to_end(rest);
            result
        } else {
            let before_left = head.advance(left as usize - 2);
            let mut start = before_left.take_next();
            let right_node = start.advance((right - left) as usize);
            let rest = right_node.take_next();
            let mut reversed = start.reverse();
            reversed.add_to_end(rest);
            head.add_to_end(reversed);

            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;

    #[test]
    fn example_1() {
        let items = vec![1, 2, 3, 4, 5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_between(nodes, 2, 4);
        assert_eq!(result.to_vec(), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn example_2() {
        let items = vec![5];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_between(nodes, 1, 1);
        assert_eq!(result.to_vec(), vec![5]);
    }

    #[test]
    fn reverse_beginning() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_between(nodes, 1, 5);
        assert_eq!(result.to_vec(), vec![6, 7, 8, 9, 10, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_end() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_between(nodes, 3, 10);
        assert_eq!(result.to_vec(), vec![10, 9, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn reverse_middle() {
        let items = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::reverse_between(nodes, 4, 9);
        assert_eq!(result.to_vec(), vec![10, 9, 8, 2, 3, 4, 5, 6, 7, 1]);
    }
}
