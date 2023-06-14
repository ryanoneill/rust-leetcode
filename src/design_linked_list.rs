use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>
}

impl Node {

    #[inline]
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

}

trait NodePointerAdditions {

    fn new(value: Node) -> Self;

    fn advance(&self, count: usize) -> Self;

    fn get_next(&self) -> Self;
    fn get_value(&self) -> i32;

    fn set_next(&mut self, value: Self); 
    fn take_next(&mut self) -> Self;

}

impl NodePointerAdditions for Option<Rc<RefCell<Node>>> {

    fn new(value: Node) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }

    fn advance(&self, count: usize) -> Self {
        let mut count = count;
        let mut result = self.clone();

        while count > 0 {
            result = Self::get_next(&result);
            count -= 1;
        }
        
        result
    }

    fn get_next(&self) -> Self {
        self.as_ref().and_then(|rc| rc.borrow().next.clone())
    }

    fn get_value(&self) -> i32 {
        match self {
            Some(rc) => {
                let node = rc.borrow();
                node.val
            }
            None => {
                -1
            }
        }
    }

    fn set_next(&mut self, value: Self) {
        match self {
            Some(rc) => {
                let mut node = rc.borrow_mut();
                node.next = value;
            }
            None => { }
        }

    }

    fn take_next(&mut self) -> Self {
        let result;
        match self {
            Some(rc) => { 
                let mut node = rc.borrow_mut();
                result = node.next.take();
            }
            None => { 
                result = None;
            }
        }
        result

    }

}

/// Design your implementation of the linked list. You can choose to use a singly or doubly linked
/// list. A node in a singly linked list should have two attributes: `val` and `next`. `val` is the
/// value of the current node, and `next` is a pointer/reference to the next node.
///
/// If you want to use the doubly linked list, you will need one more attribute `prev` to indicate
/// the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
///
/// Implement the `MyLinkedList` class:
///
/// * `MyLinkedList()` Initializes the `MyLinkedList` object.
///
/// * `int get(int index)` Get the value of the `indexth` node in the linked list. If the index is
/// invalid, return `-1`.
///
/// * `void addAtHead(int val)` Add a node of value `val` before the first element of the linked
/// list. After the insertion, the new node will be the first node of the linked list.
///
/// * `void addAtTail(int val)` Append a node of value `val` as the last element of the linked
/// list.
///
/// * `void addAtIndex(int index, int val)` Add a node of value `val` before the `indexth` node in
/// the linked list. If `index` equals the length of the linked list, the node will be appended to
/// the end of the linked list. If `index` is greater than the length, the node will not be
/// inserted.
///
/// * `void deleteAtIndex(int index)` Delete the `indexth` node in the linked list, if the index is
/// valid.
#[derive(Debug)]
struct MyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
}

impl MyLinkedList {

    fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    fn from_vec(values: Vec<i32>) -> Self {
        let mut result = Self::new();
        for val in values {
            result.add_at_tail(val)
        }
        result
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::with_capacity(self.len);
        let mut curr = self.head.clone();
        while curr != None {
            result.push(curr.get_value());
            curr = curr.get_next();
        }
        result
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.len as i32 {
            -1
        } else {
            let curr = self.head.advance(index as usize);
            curr.get_value()
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let mut node = Node::new(val);
        if self.len > 0 {
            node.next = self.head.take();
            self.head = NodePointerAdditions::new(node);
        } else {
            self.tail = NodePointerAdditions::new(node);
            self.head = self.tail.clone();
        }
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let node = Node::new(val);
        if self.len > 0 {
            let value: Option<Rc<RefCell<Node>>> = NodePointerAdditions::new(node);
            let cloned = value.clone();
            self.tail.set_next(value);
            self.tail = cloned;
            self.len += 1;
        } else {
            Self::add_at_head(self, val)
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.len as i32 {
            // Do Nothing
        } else if index == 0 {
            Self::add_at_head(self, val);
        } else if index == self.len as i32 {
            Self::add_at_tail(self, val);
        } else {
            let mut prev = self.head.clone();
            prev = prev.advance(index as usize - 1);
            let mut node = Node::new(val);
            node.next = prev.take_next();
            prev.set_next(NodePointerAdditions::new(node));
            self.len += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.len as i32 {
            // Do Nothing
        } else if self.len == 1 {
            self.head = None;
            self.tail = None;
            self.len -= 1;
        } else if index == 0 {
            let next = self.head.take_next();
            self.head = next;
            self.len -= 1;
        } else {
            let mut prev = self.head.advance(index as usize - 1);
            let next = prev.get_next().take_next();
            if next == None {
                self.tail = prev.clone();
            }
            prev.set_next(next);
            self.len -= 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;
    use super::NodePointerAdditions;

    #[test]
    fn add_at_head() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_head(1);
        my_linked_list.add_at_head(2);
        my_linked_list.add_at_head(3);
        assert_eq!(my_linked_list.len, 3);
        assert_eq!(my_linked_list.head.get_value(), 3);
        assert_eq!(my_linked_list.tail.get_value(), 1);
        assert_eq!(my_linked_list.to_vec(), vec![3,2,1]);
    }

    #[test]
    fn add_at_tail() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_tail(1);
        my_linked_list.add_at_tail(2);
        my_linked_list.add_at_tail(3);
        assert_eq!(my_linked_list.len, 3);
        assert_eq!(my_linked_list.head.get_value(), 1);
        assert_eq!(my_linked_list.tail.get_value(), 3);
        assert_eq!(my_linked_list.to_vec(), vec![1,2,3]);
    }

    #[test]
    fn add_at_index() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_index(0, 1);
        my_linked_list.add_at_index(1, 3);
        my_linked_list.add_at_index(1, 2);
        my_linked_list.add_at_index(3, 4);
        my_linked_list.add_at_index(5, 10);
        my_linked_list.add_at_index(-1, 10);
        my_linked_list.add_at_index(2, 5);
        assert_eq!(my_linked_list.len, 5);
        assert_eq!(my_linked_list.head.get_value(), 1);
        assert_eq!(my_linked_list.tail.get_value(), 4);
        assert_eq!(my_linked_list.to_vec(), vec![1,2,5,3,4]);
    }

    #[test]
    fn delete_at_index() {
        let mut my_linked_list = MyLinkedList::from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
        my_linked_list.delete_at_index(9);
        my_linked_list.delete_at_index(0);
        my_linked_list.delete_at_index(1);
        my_linked_list.delete_at_index(5);
        assert_eq!(my_linked_list.len, 6);
        assert_eq!(my_linked_list.head.get_value(), 2);
        assert_eq!(my_linked_list.tail.get_value(), 9);
        assert_eq!(my_linked_list.to_vec(), vec![2,4,5,6,7,9]);
    }

    #[test]
    fn get() {
        let my_linked_list = MyLinkedList::from_vec(vec![1,3,5,7,9]);
        assert_eq!(my_linked_list.len, 5);
        assert_eq!(my_linked_list.get(-1), -1);
        assert_eq!(my_linked_list.get(5), -1);
        assert_eq!(my_linked_list.get(0), 1);
        assert_eq!(my_linked_list.get(1), 3);
        assert_eq!(my_linked_list.get(2), 5);
        assert_eq!(my_linked_list.get(3), 7);
        assert_eq!(my_linked_list.get(4), 9);
        assert_eq!(my_linked_list.len, 5);
    }

    #[test]
    fn example_1() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_head(1);
        my_linked_list.add_at_tail(3);
        my_linked_list.add_at_index(1, 2);
        let result = my_linked_list.get(1);
        assert_eq!(result, 2);
        my_linked_list.delete_at_index(1);
        let result = my_linked_list.get(1);
        assert_eq!(result, 3);
    }

}
