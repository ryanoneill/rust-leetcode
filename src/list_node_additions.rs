use crate::list_node::ListNode;

pub trait ListNodeAdditions {
    fn new(val: i32) -> Self;
    fn with_next(val: i32, next: Self) -> Self;

    fn from_vec(items: Vec<i32>) -> Self;

    fn add_to_end(&mut self, value: Self);
    fn advance(&mut self, n: usize) -> &mut Self;
    fn get_next(&self) -> &Self;
    fn get_next_value(&self) -> i32;
    fn get_value(&self) -> i32;
    fn has_next(&self) -> bool;
    fn has_pair(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn refer_next(&mut self) -> &mut Self;
    fn refer_next_next(&mut self) -> &mut Self;
    fn remove_next(&mut self);
    fn reverse(self) -> Self;
    fn set_next(&mut self, value: Self);
    fn take_next(&mut self) -> Self;
    fn to_vec(&self) -> Vec<i32>;
}

impl ListNodeAdditions for Option<Box<ListNode>> {
    fn new(val: i32) -> Self {
        Some(Box::new(ListNode::new(val)))
    }

    fn with_next(val: i32, next: Self) -> Self {
        Some(Box::new(ListNode { val, next }))
    }

    fn from_vec(items: Vec<i32>) -> Self {
        if items.len() == 0 {
            None
        } else {
            let mut result = Self::new(items[0]);
            let mut current = &mut result;

            for i in 1..items.len() {
                let node = Self::new(items[i]);
                current.set_next(node);
                current = current.refer_next();
            }

            result
        }
    }

    fn add_to_end(&mut self, value: Self) {
        let mut current = self;
        while current.has_next() {
            current = current.refer_next();
        }
        current.set_next(value);
    }

    fn advance(&mut self, n: usize) -> &mut Self {
        let mut result = self;
        for _ in 1..=n {
            result = result.refer_next();
        }
        result
    }

    fn get_next(&self) -> &Self {
        &self.as_ref().unwrap().next
    }

    fn get_next_value(&self) -> i32 {
        self.get_next().get_value()
    }

    fn get_value(&self) -> i32 {
        self.as_ref().unwrap().val
    }

    fn has_next(&self) -> bool {
        self.get_next().is_some()
    }

    fn has_pair(&self) -> bool {
        !self.is_empty() && self.has_next()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_none()
    }

    fn len(&self) -> usize {
        let mut result = 0;
        let mut current = self;

        while !current.is_empty() {
            result += 1;
            current = &current.get_next();
        }

        result
    }

    fn refer_next(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().next
    }

    fn refer_next_next(&mut self) -> &mut Self {
        self.advance(2)
    }

    fn remove_next(&mut self) {
        let mut removed = self.take_next();
        let after = removed.take_next();
        self.set_next(after);
    }

    fn reverse(self) -> Self {
        let mut previous = None;
        let mut current = self;

        while current.is_some() {
            let next_node = current.take_next();
            current.set_next(previous);
            previous = current;
            current = next_node;
        }

        previous
    }

    fn set_next(&mut self, value: Self) {
        self.as_mut().unwrap().next = value
    }

    fn take_next(&mut self) -> Self {
        self.as_mut().unwrap().next.take()
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![];
        let mut current = self;

        while current.is_some() {
            result.push(current.get_value());
            current = current.get_next();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::ListNodeAdditions;
    use crate::list_node::ListNode;

    #[test]
    fn empty_list() {
        let result: Option<Box<ListNode>> = ListNodeAdditions::from_vec(vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn one_item_list() {
        let result: Option<Box<ListNode>> = ListNodeAdditions::from_vec(vec![1]);
        assert_eq!(result, Some(Box::new(ListNode::new(1))));
    }

    #[test]
    fn two_item_list() {
        let result: Option<Box<ListNode>> = ListNodeAdditions::from_vec(vec![3, 4]);
        let four = ListNodeAdditions::new(4);
        let three = ListNodeAdditions::with_next(3, four);
        assert_eq!(result, three);
    }

    #[test]
    fn three_item_list() {
        let result: Option<Box<ListNode>> = ListNodeAdditions::from_vec(vec![1, 3, 5]);
        let five: Option<Box<ListNode>> = ListNodeAdditions::new(5);
        let three: Option<Box<ListNode>> = ListNodeAdditions::with_next(3, five);
        let one: Option<Box<ListNode>> = ListNodeAdditions::with_next(1, three);

        assert_eq!(result, one);
    }

    #[test]
    fn vec_to_nodes_to_vec() {
        let items = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        let nodes: Option<Box<ListNode>> = ListNodeAdditions::from_vec(items);
        let result = nodes.to_vec();

        assert_eq!(result, vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10]);
    }
}
