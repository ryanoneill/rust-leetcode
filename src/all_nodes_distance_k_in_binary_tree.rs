use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

/// Given the root of a binary tree, the value of a target node `target`, and
/// an integer `k`, return an array of the values of all nodes that have a
/// distance `k` from the target node.
///
/// You can return the answer in any order.
struct Solution;

impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let result: Vec<i32>;

        let parents = root.find_parents();
        let children = root.find_children();
        let start_value = target.get_value();

        match start_value {
            Some(start) => {
                let mut queue = VecDeque::new();
                let mut seen = HashSet::new();
                let mut distance = 0;

                queue.push_back(start);
                seen.insert(start);
                while !queue.is_empty() && distance < k {
                    let len = queue.len();
                    for _ in 0..len {
                        let node = queue.pop_front().unwrap();
                        if parents.contains_key(&node) {
                            let parent = parents[&node];
                            if !seen.contains(&parent) {
                                seen.insert(parent);
                                queue.push_back(parent);
                            }
                        }
                        if children.contains_key(&node) {
                            let left_right = &children[&node];
                            for child in left_right {
                                if !seen.contains(child) {
                                    seen.insert(*child);
                                    queue.push_back(*child);
                                }
                            }
                        }
                    }
                    distance += 1;
                }

                result = queue.into_iter().collect();
            }
            None => result = Vec::new(),
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_node::TreeNode;
    use crate::tree_node_additions::TreeNodeAdditions;

    #[test]
    fn example_1() {
        let root = tree!("[3,5,1,6,2,0,8,null,null,7,4]");
        let target = root.clone_left();
        let k = 2;
        let mut result = Solution::distance_k(root, target, k);
        result.sort();
        assert_eq!(result, vec![1, 4, 7]);
    }

    #[test]
    fn example_2() {
        let root = tree!("[1]");
        let target = root.clone();
        let k = 3;
        let result = Solution::distance_k(root, target, k);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn real_world_1() {
        let root = tree!("[0,2,1,null,null,3]");
        let target = root.clone_right().clone_left();
        assert_eq!(target.get_value(), Some(3));
        let k = 3;
        let result = Solution::distance_k(root, target, k);
        assert_eq!(result, vec![2]);
    }
}
