use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the head of a linked list and a value `x`, partition it such that all nodes less than `x`
/// come before nodes greater than or equal to `x`.
///
/// You should preserve the original relative order of the nodes in each of the two partitions.
struct Solution;

impl Solution {

    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut list_small: Option<Box<ListNode>> = None;
        let mut last_small: &mut Option<Box<ListNode>> = &mut list_small;
        let mut list_big: Option<Box<ListNode>> = None;
        let mut last_big: &mut Option<Box<ListNode>> = &mut list_big;

        let mut head = head;
        while head.is_some() {
            let next = head.take_next();
            let current = head;
            head = next;

            if current.get_value() < x {
                if last_small.is_none() {
                    list_small = current;
                    last_small = &mut list_small;
                } else {
                    last_small.set_next(current);
                    last_small = last_small.refer_next();
                }
            } else {
                if last_big.is_none() {
                    list_big = current;
                    last_big = &mut list_big;
                } else {
                    last_big.set_next(current);
                    last_big = last_big.refer_next();
                }
            }

        }

        let result;
        if last_small.is_some() {
            last_small.set_next(list_big);
            result = list_small;
        } else {
            result = list_big;
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
        let items = vec![1,4,3,2,5,2];
        let x = 3;
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::partition(nodes, x);
        assert_eq!(result.to_vec(), vec![1,2,2,4,3,5]);
    }

    #[test]
    fn example_2() {
        let items = vec![2, 1];
        let x = 2;
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::partition(nodes, x);
        assert_eq!(result.to_vec(), vec![1,2]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let x = 0;
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::partition(nodes, x);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_4() {
        let items = vec![1];
        let x = 0;
        let nodes = ListNodeAdditions::from_vec(items);
        let result = Solution::partition(nodes, x);
        assert_eq!(result.to_vec(), vec![1]);
    }

}
