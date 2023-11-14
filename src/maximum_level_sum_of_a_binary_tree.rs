use crate::tree_node::TreeNode;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, the level of its root is `1`, the level
/// of its children is `2`, and so on.
///
/// Return the smallest level `x` such that the sum of all the values of nodes
/// at level `x` is maximal.
struct Solution;

impl Solution {

    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 { 
        let mut max_sum: i64 = i64::MIN;
        let mut current_sum: i64;
        let mut current_level: i32 = 0;
        let mut result = 0;
        let mut queue = VecDeque::new();

        queue.push_back(root.clone());
        while !queue.is_empty() {
            let q = queue.len();
            current_sum = 0;
            current_level += 1;

            for _ in 0..q {
                let item = queue.pop_front().unwrap();
                match item {
                    Some(rc) => {
                        let node: Ref<TreeNode> = rc.borrow();
                        current_sum += node.val as i64;
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

            if current_sum > max_sum {
                max_sum = current_sum;
                result = current_level;
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[1,7,0,7,-8,null,null]");
        let result = Solution::max_level_sum(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let root = tree!("[989,null,10250,98693,-89388,null,null,null,-32127]");
        let result = Solution::max_level_sum(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn negative_sums() {
        let root = tree!("[-100,-200,-300,-20,-5,-10,null]");
        let result = Solution::max_level_sum(root);
        assert_eq!(result, 3);
    }

}
