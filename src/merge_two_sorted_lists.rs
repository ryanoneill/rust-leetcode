use crate::list_node::ListNode;
use crate::list_node_additions::ListNodeAdditions;

pub struct Solution;

impl Solution {

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        if list1.is_empty() { list2 }
        else if list2.is_empty() { list1 }
        else {
            let mut list1 = list1;
            let mut list2 = list2;
            let mut result = None;
            let mut current = &mut result;
            let mut first = true;

            while !list1.is_empty() || !list2.is_empty() {
                if first {
                    let list1_value = list1.get_value();
                    let list2_value = list2.get_value();

                    if list1_value <= list2_value {
                        result = list1;
                        current = &mut result;
                        list1 = current.take_next();
                    } else {
                        result = list2;
                        current = &mut result;
                        list2 = current.take_next();
                    }

                    first = false;
                } else if list1.is_empty() {
                    current.set_next(list2);
                    list2 = None;
                } else if list2.is_empty() {
                    current.set_next(list1);
                    list1 = None;
                } else {
                    let list1_value = list1.get_value();
                    let list2_value = list2.get_value();

                    if list1_value <= list2_value {
                        current.set_next(list1);
                        current = current.refer_next();
                        list1 = current.take_next();
                    } else {
                        current.set_next(list2);
                        current = current.refer_next();
                        list2 = current.take_next();
                    }
                }
            }

            result
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::list_node_additions::ListNodeAdditions;
    use super::Solution;

    #[test]
    fn example_1() {
        let items1 = vec![1, 2, 4];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![1, 3, 4];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![1,1,2,3,4,4]);
    }

    #[test]
    fn example_2() {
        let items1 = vec![];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![]);
    }

    #[test]
    fn example_3() {
        let items1 = vec![];
        let list1 = ListNodeAdditions::from_vec(items1);
        let items2 = vec![0];
        let list2 = ListNodeAdditions::from_vec(items2);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result.to_vec(), vec![0]);
    }

}
