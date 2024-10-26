use crate::kth_largest::KthLargest;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// You are given the `root` of a binary tree and a positive integer `k`.
///
/// The *level sum* in the tree is the sum of the values of the nodes that are on the *same* level.
///
/// Return the `kth` *largest* level sum in the tree (not necessarily distinct). If there are fewer
/// than `k` levels in the tree, return `-1`.
///
/// Note that two nodes are on the same level if they have the same distance from the root.
struct Solution;

impl Solution {

    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut kth_largest = KthLargest::new(k as usize);

        let mut sum: i64;
        let mut queue = VecDeque::new();

        queue.push_back(root.clone());
        while !queue.is_empty() {
            let q = queue.len();
            sum = 0;

            for _ in 0..q {
                let item = queue.pop_front().unwrap();
                match item {
                    Some(rc) => {
                        let node = rc.borrow();
                        sum += node.val as i64;
                        if node.left.is_some() {
                            queue.push_back(node.left.clone());
                        }
                        if node.right.is_some() {
                            queue.push_back(node.right.clone());
                        }
                    }
                    None => { }
                }
            }

            kth_largest.add(sum);
        }

        if kth_largest.full() {
            kth_largest.peek()
        } else {
            -1
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[5,8,9,2,1,3,7,4,6]");
        let k = 2;
        let result = Solution::kth_largest_level_sum(root, k);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1,2,null,3]");
        let k = 1;
        let result = Solution::kth_largest_level_sum(root, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_no() {
        let root = tree!("[1,2,3]");
        let k = 3;
        let result = Solution::kth_largest_level_sum(root, k);
        assert_eq!(result, -1);
    }

}
