use crate::list_node::ListNode;

pub trait ListNodeAdditions {

    fn add_to_end(&mut self, value: Self);
    fn advance(&mut self, n: usize) -> &mut Self;
    fn has_next(&self) -> bool;
    fn refer_next(&mut self) -> &mut Self;
    fn reverse(self) -> Self;
    fn set_next(&mut self, value: Self);
    fn take_next(&mut self) -> Self;

}

impl ListNodeAdditions for Option<Box<ListNode>> {

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

    fn has_next(&self) -> bool {
        self.as_ref().unwrap().next.is_some()
    }

    fn refer_next(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().next
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

}
