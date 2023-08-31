use crate::list_node::ListNode;
use std::collections::HashSet;
use std::iter::FromIterator;

/// You are given the `head` of a linked list containing unique integer values and an integer array
/// `nums` that is a subset of the linked list values.
///
/// Return the number of connected components in `nums` where two values are connected if they
/// appear 
struct Solution;

impl Solution {

    fn refer_next(item: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
        if item.is_some() {
            &item.as_ref().unwrap().next
        } else {
            item
        }
    }

    fn get_value(item: &Option<Box<ListNode>>) -> i32 {
        if item.is_some() {
            item.as_ref().unwrap().val
        } else {
            -1
        }
    }

    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut minus = 0;
        let numset: HashSet<i32> = HashSet::from_iter(nums);

        let mut current = &head;
        let mut last = Self::get_value(current);

        current = Self::refer_next(current);
        while current.is_some() {
            let value = Self::get_value(current);
            if numset.contains(&last) && numset.contains(&value) {
                minus += 1;
            }
            last = value;
            current = Self::refer_next(current);
        }

        n - minus
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items = vec![0,1,2,3];
        let nums = vec![0,1,3];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::num_components(head, nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let items = vec![0,1,2,3,4];
        let nums = vec![0,3,1,4];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::num_components(head, nums);
        assert_eq!(result, 2);
    }

}
