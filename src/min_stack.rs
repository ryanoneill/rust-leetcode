#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Element {
    val: i32,
    min: i32,
}

impl Element {

    fn new(val: i32, min: i32) -> Self {
        Self { val, min }
    }

}

/// Design a stack that supports push, pop, top, and retrieving the minimum
/// element in constant time.
///
/// Implement the `MinStack` class:
///
/// * `MinStack()` initializes the stack object.
/// * `void push(int val)` pushes the element `val` on the the stack.
/// * `void pop()` removes the element on the top of the stack.
/// * `int top()` gets the top element of the stack.
/// * `int getMin()` retrieves the minimum element in the stack.
///
/// You must implement a solution with `O(1)` time complexity for each
/// funtion.
struct MinStack {
    items: Vec<Element>
}

impl MinStack {

    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let element;
        if self.items.is_empty() {
            element = Element::new(val, val);
        } else {
            let min = self.get_min();
            element = Element::new(val, val.min(min));
        }
        self.items.push(element);
    }

    fn pop(&mut self) {
        self.items.pop();
    }

    fn top(&self) -> i32 {
        let n = self.items.len();
        if n == 0 { 0 }
        else { self.items[n-1].val }
    }

    fn get_min(&self) -> i32 {
        let n = self.items.len();
        if n == 0 { 0 }
        else { self.items[n-1].min }
    }

}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn example_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        let result = min_stack.get_min();
        assert_eq!(result, -3);
        min_stack.pop();
        let result = min_stack.top();
        assert_eq!(result, 0);
        let result = min_stack.get_min();
        assert_eq!(result, -2);
    }

}
