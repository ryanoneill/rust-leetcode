use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

struct Solution;

impl Solution {

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let k = k as usize;
        let mut n = head.len();

        let mut result;

        if n >= k {
            let mut rest = head.split_at(k);
            result = head.reverse();
            let mut current = &mut result;
            current = current.advance(k-1);
            n -= k;

            while n >= k {
                let next = rest.split_at(k);
                current.set_next(rest.reverse());
                current = current.advance(k);
                n -= k;
                rest = next;
            }

            current.set_next(rest);
        } else {
            result = head;
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
        let items = vec![1,2,3,4,5];
        let head = ListNodeAdditions::from_vec(items);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(result.to_vec(), vec![2,1,4,3,5]);
    }

    #[test]
    fn example_2() {
        let items = vec![1,2,3,4,5];
        let head = ListNodeAdditions::from_vec(items);
        let k = 3;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(result.to_vec(), vec![3,2,1,4,5]);
    }

    #[test]
    fn example_bad_k() {
        let items = vec![1,2,3];
        let head = ListNodeAdditions::from_vec(items);
        let k = 4;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(result.to_vec(), vec![1,2,3]);
    }

    #[test]
    fn example_equal_k() {
        let items = vec![1,2,3,4,5,6];
        let head = ListNodeAdditions::from_vec(items);
        let k = 2;
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(result.to_vec(), vec![2,1,4,3,6,5]);
    }

}
