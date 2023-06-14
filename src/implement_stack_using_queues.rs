use std::collections::VecDeque;

/// Implement a last-in-first-out (LIFO) stack using only two queues. The implemented stack should
/// support all the functions of a normal stack (`push`, `top`, `pop`, and `empty`).
///
/// Implement the `MyStack` class:
///
/// * `void push(int x)` Pushes element x to the top of the stack.
///
/// * `int pop()` Removes the element on the top of the stack and returns it.
///
/// * `int top()` Returns the element on the top of the stack.
///
/// * `boolean empty()` Returns `true` if the stack is empty, `false` otherwise.
///
/// Notes:
///
/// * You must use only standard operations of a queue, which means that only `push to back`,
/// `peek/pop from front`, `size`, and `is empty` operations are valid.
///
/// * Depending on your language, the queue may not be supported natively. You may simulate a queue
/// using a list or deque (double-ended queue) as long as you use only a queue's standard
/// operations.
struct MyStack {
    items1: VecDeque<i32>,
    items2: VecDeque<i32>,
    items1_is_front: bool,
}

impl MyStack {

    fn new() -> Self {
        Self { items1: VecDeque::new(), items2: VecDeque::new(), items1_is_front: true }
    }

    fn push(&mut self, x: i32) {
        let &mut back;
        let &mut front;

        if self.items1_is_front {
            front = &mut self.items1;
            back = &mut self.items2;
        } else {
            front = &mut self.items2;
            back = &mut self.items1;
        }

        back.push_back(x);

        while !front.is_empty() {
            let item = front.pop_front().unwrap();
            back.push_back(item);
        }

        self.items1_is_front = !self.items1_is_front;
    }

    fn pop(&mut self) -> i32 {
        let &mut front;

        if self.items1_is_front {
            front = &mut self.items1;
        } else {
            front = &mut self.items2;
        }

        front.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        let front: &VecDeque<i32>;

        if self.items1_is_front {
            front = &self.items1;
        } else {
            front = &self.items2;
        }

        front[0]
    }

    fn empty(&self) -> bool {
        self.items1.is_empty() && self.items2.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::MyStack;

    #[test]
    fn example_1() {
        let mut my_stack = MyStack::new();
        my_stack.push(1);
        my_stack.push(2);
        let result = my_stack.top();
        assert_eq!(result, 2);
        let result = my_stack.pop();
        assert_eq!(result, 2);
        let result = my_stack.empty();
        assert!(!result);
    }

}
