use std::iter::FromIterator;

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.items.get(self.len() - 1)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn push(&mut self, elem: T) {
        self.items.push(elem);
    }

    pub fn reverse(&mut self) {
        let mut new_items = Vec::with_capacity(self.len());
        while !self.is_empty() {
            new_items.push(self.pop().unwrap());
        }
        self.items = new_items;
    }
}

impl Stack<&str> {
    pub fn join(&self, sep: &str) -> String {
        self.items.join(sep)
    }
}

impl Stack<char> {
    pub fn to_string(&self) -> String {
        String::from_iter(self.items.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn empty_when_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn not_empty_when_added_to() {
        let mut stack = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn multiple_elements_can_be_added() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        stack.push('c');
        stack.push('d');
        stack.push('e');
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 5);
    }

    #[test]
    fn peek_on_empty_returns_none() {
        let stack: Stack<i32> = Stack::new();
        let value = stack.peek();
        assert_eq!(value, None);
    }

    #[test]
    fn peek_retrieves_the_last_element() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let value = stack.peek();
        assert_eq!(value, Some(&3));
    }

    #[test]
    fn pop_on_empty_returns_none() {
        let mut stack: Stack<String> = Stack::new();
        let value = stack.pop();
        assert_eq!(value, None);
    }

    #[test]
    fn pop_on_non_empty_returns_owned_value() {
        let mut stack: Stack<String> = Stack::new();
        stack.push("hello".to_string());
        stack.push("goodbye".to_string());
        let value = stack.pop();
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "goodbye");
    }
}
