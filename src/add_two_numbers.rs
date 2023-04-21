use crate::list_node::ListNode;

/// You are given two non-empty linked lists representing two non-negative
/// integers. The digits are stored in reverse order, and each of their nodes
/// contains a single digit. Add the two numbers and return the sum as a linked
/// list.
///
/// You may assume the two numbers do not contain any leading zero, except the
/// number 0 itself.
struct Solution;

impl Solution {

    fn add_values(val1: i32, val2: i32, overflow: bool) -> (i32, bool) {
        let mut sum = val1 + val2;
        if overflow {
            sum += 1;
        }

        if sum >= 10 {
            (sum - 10, true)
        } else {
            (sum, false)
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut result = None;
        let mut current = &mut result;

        let mut overflow = false;

        loop {
            match (l1, l2) {
                (None, None) => {
                    let (sum, _) = Self::add_values(0, 0, overflow);
                    if sum > 0 {
                        let node = Some(Box::new(ListNode::new(sum)));
                        if current.as_mut().is_none() {
                            result = node;
                        } else {
                            current.as_mut().unwrap().next = node;
                        }
                    }

                    break;
                }
                (node1, node2) => {
                    let val1 = node1.as_ref().map(|n1| n1.val).unwrap_or(0);
                    let val2 = node2.as_ref().map(|n2| n2.val).unwrap_or(0);

                    let (sum, carry) = Self::add_values(val1, val2, overflow);
                    overflow = carry;

                    let node = Some(Box::new(ListNode::new(sum)));
                    if current.as_mut().is_none() {
                        result = node;
                        current = &mut result;
                    } else {
                        current.as_mut().unwrap().next = node;
                        current = &mut current.as_mut().unwrap().next;
                    }

                    l1 = node1.and_then(|n1| n1.next);
                    l2 = node2.and_then(|n2| n2.next);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use super::Solution;

    #[test]
    fn example_1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn example_2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0]);
    }

    #[test]
    fn example_3() {
        let l1 = ListNode::from_vec(vec![9,9,9,9,9,9,9]);
        let l2 = ListNode::from_vec(vec![9,9,9,9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![8,9,9,9,0,0,0,1]);
    }

}
