use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// This is an implementation of a doubly linked list where the node infrastructure is externalized.
// It is intentionally built this way as a precursor to implementation within the LRUCache question.

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {

    fn new(key: i32, value: i32) -> Self {
        Self { key, value, prev: None, next: None }
    }

}

impl fmt::Debug for Node {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("prev", &self.prev.get_key())
            .field("next", &self.next.get_key())
            .finish()
    }

}

trait NodeAdditions {
    fn get_key(&self) -> Option<i32>;
    fn has_key_value(&self, key: i32, value: i32) -> bool;

    fn set_prev(&self, prev: Self);
    fn set_next(&self, next: Self);

    fn take_prev(&self) -> Self;
    fn take_next(&self) -> Self;
}

impl NodeAdditions for Option<Rc<RefCell<Node>>> {

    fn get_key(&self) -> Option<i32> {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                Some(node.key)
            }
            None => None
        }
    }

    fn has_key_value(&self, key: i32, value: i32) -> bool {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                node.key == key && node.value == value
            }
            None => false
        }

    }

    fn set_prev(&self, prev: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.prev = prev;
            }
            None => { } // do nothing
        }
    }

    fn set_next(&self, next: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.next = next;
            }
            None => { } // do nothing
        }
    }

    fn take_prev(&self) -> Self {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.prev.take()
            }
            None => {
                None
            }
        }
    }

    fn take_next(&self) -> Self {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.next.take()
            }
            None => {
                None
            }
        }
    }

}

trait NodeCellAdditions {

    fn wrapped(self) -> Option<Rc<RefCell<Self>>>;

}

impl NodeCellAdditions for Node {

    fn wrapped(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }

}

struct LinkedList {
    front: Option<Rc<RefCell<Node>>>,
    back: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {

    fn new() -> Self {
        let front = Node::new(i32::MIN, i32::MIN).wrapped();
        let back = Node::new(i32::MAX, i32::MAX).wrapped();
        
        front.set_next(back.clone());
        back.set_prev(front.clone());

        Self { front, back }
    }

    fn push_back(&mut self, item: Option<Rc<RefCell<Node>>>) {
        if item.is_some() {
            let prev = self.back.take_prev();
            let next = prev.take_next();

            prev.set_next(item.clone());
            next.set_prev(item.clone());

            item.set_prev(prev);
            item.set_next(next);
        }
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<Node>>> {
        let result = self.front.take_next();
        if result == self.back {
            self.front.set_next(result);
            None
        } else {
            let result_next = result.take_next();
            let result_prev = result.take_prev();

            result_next.set_prev(result_prev);
            self.front.set_next(result_next);

            result
        }
    }

    fn remove(&mut self, item: &Option<Rc<RefCell<Node>>>) {
        if item.is_some() {
            let prev = item.take_prev();
            let next = item.take_next();

            prev.set_next(next.clone());
            next.set_prev(prev.clone());
        }
    }

}

#[cfg(test)]
mod tests {
    use super::LinkedList;
    use super::Node;
    use super::NodeAdditions;
    use super::NodeCellAdditions;

    #[test]
    fn new_check() {
        let mut linked_list = LinkedList::new();
        let result = linked_list.pop_front();
        assert_eq!(result, None);
    }

    #[test]
    fn push_pop() {
        let mut linked_list = LinkedList::new();
        let item = Node::new(1234, 5678).wrapped();
        linked_list.push_back(item);

        let result = linked_list.pop_front();
        assert!(result.is_some());
        assert!(result.has_key_value(1234, 5678));

        let item = Node::new(2345, 6789).wrapped();
        linked_list.push_back(item);

        let result = linked_list.pop_front();
        assert!(result.has_key_value(2345, 6789));
    }

    #[test]
    fn push_remove_pop() {
        let mut linked_list = LinkedList::new();
        let item = Node::new(1234, 5678).wrapped();
        linked_list.push_back(item.clone());

        linked_list.remove(&item);
        let result = linked_list.pop_front();
        assert!(result.is_none());

        linked_list.push_back(item);
        let result = linked_list.pop_front();
        assert!(result.has_key_value(1234, 5678));
    }

}
