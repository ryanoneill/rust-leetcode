use std::iter::FromIterator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }

    pub fn from_vec(items: Vec<i32>) -> Option<Box<ListNode>> {
        if items.len() == 0 {
            None
        } else {
            let mut result = Some(Box::new(ListNode::new(items[0])));
            let mut current = &mut result;

            for i in 1..items.len() {
                let node = Some(Box::new(ListNode::new(items[i])));

                current.as_mut().map(|cnode| {
                    cnode.next = node;
                });
                current = &mut current.as_mut().unwrap().next;

            }

            result
        }
    }

    pub fn to_vec(nodes: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nodes = nodes;
        let mut result = vec![];
        let mut current = &mut nodes;

        while let Some(box_node) = current {
            result.push(box_node.val);
            current = &mut current.as_mut().unwrap().next;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_next() {
        let one_a = ListNode::new(1);
        let two_a = ListNode::with_next(2, Some(Box::new(one_a)));

        let one_b = ListNode::new(1);
        let two_b = ListNode { val: 2, next: Some(Box::new(one_b)) };

        assert_eq!(two_a, two_b);
    }

    #[test]
    fn empty_list() {
        let result = ListNode::from_vec(vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn one_item_list() {
        let result = ListNode::from_vec(vec![1]);
        assert_eq!(result, Some(Box::new(ListNode::new(1))));
    }

    #[test]
    fn two_item_list() {
        let result = ListNode::from_vec(vec![3, 4]);
        let four = Some(Box::new(ListNode::new(4)));
        let three = Some(Box::new(ListNode { val: 3, next: four }));
        assert_eq!(result, three);
    }

    #[test]
    fn three_item_list() {
        let result = ListNode::from_vec(vec![1, 3, 5]);
        let five = ListNode::new(5);
        let three = ListNode::with_next(3, Some(Box::new(five)));
        let one = ListNode::with_next(1, Some(Box::new(three)));

        assert_eq!(result, Some(Box::new(one)));
    }

    #[test]
    fn vec_to_nodes_to_vec() {
        let items = vec![1,3,5,7,9,2,4,6,8,10];
        let nodes = ListNode::from_vec(items);
        let result = ListNode::to_vec(nodes);

        assert_eq!(result, vec![1,3,5,7,9,2,4,6,8,10]);
    }

}
