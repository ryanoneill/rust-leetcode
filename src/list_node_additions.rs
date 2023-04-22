use crate::list_node::ListNode;

pub trait ListNodeAdditions {

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

}
