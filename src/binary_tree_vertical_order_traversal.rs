use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the `root` of a binary tree, return the vertical order traversal of its nodes' values.
/// (i.e., from top to bottom, column by column).
///
/// If two nodes are in the same row and column, the order should be from left to right.
struct Solution;

impl Solution {

    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut values: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((0, root));
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (item_x, item) = queue.pop_front().unwrap();
                match item {
                    Some(rc) => {
                        let node = rc.borrow();
                        values
                            .entry(item_x)
                            .and_modify(|nums| { nums.push(node.val); })
                            .or_insert(vec![node.val]);

                        if node.left.is_some() {
                            queue.push_back((item_x - 1, node.left.clone()));
                        }
                        if node.right.is_some() {
                            queue.push_back((item_x + 1, node.right.clone()));
                        }
                    }
                    None => { }
                }

            }

        }

        let mut results = Vec::new();

        if !values.is_empty() {
            let min = *values.keys().min().unwrap();
            let n = values.len() as i32;

            for i in min..(min+n) {
                let items = values.remove(&i).unwrap();
                results.push(items);
            }
        }
        
        results
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree!("[3,9,20,null,null,15,7]");
        let result = Solution::vertical_order(root);
        assert_eq!(result, vec![
            vec![9],
            vec![3, 15],
            vec![20],
            vec![7],
        ]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[3,9,8,4,0,1,7]");
        let result = Solution::vertical_order(root);
        assert_eq!(result, vec![
            vec![4],
            vec![9],
            vec![3,0,1],
            vec![8],
            vec![7],
        ]);
    }

    #[test]
    fn example_3() {
        let root = tree!("[1,2,3,4,10,9,11,null,5,null,null,null,null,null,null,null,6]");
        let result = Solution::vertical_order(root);
        assert_eq!(result, vec![
            vec![4],
            vec![2,5],
            vec![1,10,9,6],
            vec![3],
            vec![11],
        ]);
    }

}
