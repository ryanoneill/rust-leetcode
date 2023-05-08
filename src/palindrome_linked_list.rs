/// Given the head of a singly linked list, return `true` if it is a
/// palindrome or `false` otherwise.
use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

struct Solution;

impl Solution {

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let n = head.len();
        let is_odd = n % 2 == 1;
        let mut result = true;
        let mut stack = Vec::new();

        let mut current = &head;
        for _ in 0..n/2 {
            if current.is_some() {
                stack.push(current.get_value());
                current = current.get_next();
            }
        }

        if is_odd && current.is_some() {
            current = current.get_next();
        }

        for _ in 0..n/2 {
            if current.is_some() {
                if !stack.is_empty() {
                    let expected = stack.pop().unwrap();
                    let value = current.get_value();
                    if expected != value {
                        result = false;
                        break;
                    }
                } else {
                    result = false;
                    break;
                }
                current = current.get_next();
            }
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
        let items = vec![1,2,2,1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::is_palindrome(head);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let items = vec![1,2];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::is_palindrome(head);
        assert!(!result);
    }

    #[test]
    fn odd() {
        let items = vec![1,3,5,3,1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::is_palindrome(head);
        assert!(result);
    }

    #[test]
    fn single() {
        let items = vec![1];
        let head = ListNodeAdditions::from_vec(items);
        let result = Solution::is_palindrome(head);
        assert!(result);
    }

}


