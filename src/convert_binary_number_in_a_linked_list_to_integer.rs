use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given `head` which is a reference node to a singly-linked list. The value of each node in the
/// linked list is either `0` or `1`. The linked list holds the binary representation of a number.
///
/// Return the decimal value of the number in the linked list.
///
/// The most significant bit is at the head of the linked list.
struct Solution;

impl Solution {

    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut current = &head;

        while current.is_some() {
            result *= 2;
            result += current.get_value();
            current = current.get_next();
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
        let items = vec![1,0,1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::get_decimal_value(head);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let items = vec![0];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::get_decimal_value(head);
        assert_eq!(result, 0);
    }
}
