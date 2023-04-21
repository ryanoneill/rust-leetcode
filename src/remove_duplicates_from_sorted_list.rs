use crate::list_node::ListNode;

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

    fn get_value(node: &mut Option<Box<ListNode>>) -> i32 {
        node.as_ref().unwrap().val
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        if head != None {
            let mut previous = &mut head;
            let mut last_value = Self::get_value(previous);
            let mut current = Self::take_next(previous);

            while let Some(node) = current.as_ref() {
                if node.val != last_value {
                    last_value = node.val;
                    Self::set_next(previous, current);
                    previous = Self::refer_next(previous);
                    current = Self::take_next(previous);
                } else {
                    current = Self::take_next(&mut current);
                }
            }

        }

        head
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;

    use super::Solution;


    #[test]
    fn example_1() {
        let head = ListNode::from_vec(vec![1, 1, 2]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(ListNode::to_vec(result), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let head = ListNode::from_vec(vec![1, 1, 2, 3, 3]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn all_ones() {
        let head = ListNode::from_vec(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(ListNode::to_vec(result), vec![1]);
    }

    #[test]
    fn one_one_two_two() {
        let head = ListNode::from_vec(vec![1, 1, 2, 2, 1, 1, 2, 2]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 1, 2]);
    }
}
