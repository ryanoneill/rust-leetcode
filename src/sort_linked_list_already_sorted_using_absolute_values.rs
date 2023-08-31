use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a single linked list that is sorted in non-decreasing order using the
/// absolute values of its nodes, return the list sorted in non-decreasing order using the actual
/// values of its nodes.
struct Solution;

impl Solution {

    pub fn sort_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut positives: Option<Box<ListNode>> = None;
        let mut negatives: Option<Box<ListNode>> = None;

        let mut pos_current = &mut positives;
        let mut neg_current = &mut negatives;

        let mut current = head;
        while current.is_some() {
            let next = current.take_next();
            if current.get_value() < 0 {
                if neg_current.is_none() {
                    negatives = current;
                    neg_current = &mut negatives;
                } else {
                    neg_current.set_next(current);
                    neg_current = neg_current.refer_next();
                }
            } else {
                if pos_current.is_none() {
                    positives = current;
                    pos_current = &mut positives;
                } else {
                    pos_current.set_next(current);
                    pos_current = pos_current.refer_next();
                }
            }
            current = next;
        }

        let result;
        if negatives.is_some() {
            negatives = negatives.reverse();
            negatives.add_to_end(positives);
            result = negatives;
        } else {
            result = positives;
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
        let items = vec![0,2,-5,5,10,-10];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_linked_list(head);
        assert_eq!(result.to_vec(), vec![-10,-5,0,2,5,10]);
    }

    #[test]
    fn example_2() {
        let items = vec![0,1,2];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_linked_list(head);
        assert_eq!(result.to_vec(), vec![0,1,2]);
    }

    #[test]
    fn example_3() {
        let items = vec![1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::sort_linked_list(head);
        assert_eq!(result.to_vec(), vec![1]);
    }

}
