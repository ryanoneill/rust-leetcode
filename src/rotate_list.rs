use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

/// Given the `head` of a linked list, rotate the list to the right by `k` places.
struct Solution;

impl Solution {

    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let n = head.len();
        let k = k as usize;

        let result;

        if n == 0 || k % n == 0 {
            result = head;
        } else {
            let index = n - (k % n);
            let mut rest = head;
            let mut front = rest.split_at(index);
            front.add_to_end(rest);
            result = front;
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
        let items = vec![1, 2, 3, 4, 5];
        let head = ListNodeAdditions::from_vec(items);
        let k = 2;
        let rotated = Solution::rotate_right(head, k);
        let result = rotated.to_vec();
        assert_eq!(result, vec![4,5,1,2,3]);
    }

    #[test]
    fn example_2() {
        let items = vec![0, 1, 2];
        let head = ListNodeAdditions::from_vec(items);
        let k = 4;
        let rotated = Solution::rotate_right(head, k);
        let result = rotated.to_vec();
        assert_eq!(result, vec![2, 0, 1]);
    }

    #[test]
    fn example_3() {
        let items = vec![];
        let head = ListNodeAdditions::from_vec(items);
        let k = 0;
        let rotated = Solution::rotate_right(head, k);
        let result = rotated.to_vec();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_4() {
        let items = vec![1];
        let head = ListNodeAdditions::from_vec(items);
        let k = 0;
        let rotated = Solution::rotate_right(head, k);
        let result = rotated.to_vec();
        assert_eq!(result, vec![1]);
    }

}
