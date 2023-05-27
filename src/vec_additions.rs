/// Useful functions that make working with Vecs as data structures easier.
pub trait VecAdditions<T> {
    fn peek_last(&self) -> Option<&T>;
    fn pop_while<F: Fn(&T) -> bool>(&mut self, p: F);
}

impl<T> VecAdditions<T> for Vec<T> {
    fn peek_last(&self) -> Option<&T> {
        let len = self.len();
        let last_index = if len == 0 { 0 } else { len - 1 };
        self.get(last_index)
    }

    fn pop_while<F>(&mut self, p: F)
    where
        F: Fn(&T) -> bool,
    {
        loop {
            match self.peek_last() {
                Some(value) if p(value) => {
                    self.pop();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VecAdditions;

    #[test]
    fn peek_last_empty() {
        let items: Vec<i32> = vec![];
        let result = items.peek_last();
        assert_eq!(result, None);
    }

    #[test]
    fn peek_last_one_element() {
        let items = vec![10];
        let result = items.peek_last();
        assert_eq!(result, Some(&10));
    }

    #[test]
    fn peek_last_many_elements() {
        let items = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let result = items.peek_last();
        assert_eq!(result, Some(&'g'));
    }

    #[test]
    fn pop_while_empty() {
        let mut items: Vec<i32> = vec![];
        items.pop_while(|&x| x == 0);
        assert_eq!(items, vec![]);
    }

    #[test]
    fn pop_while_false() {
        let mut items = vec![2, 4, 6, 8];
        items.pop_while(|_| false);
        assert_eq!(items, vec![2, 4, 6, 8]);
    }

    #[test]
    fn pop_while_greater_than_five() {
        let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        items.pop_while(|&x| x > 5);
        assert_eq!(items, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn pop_while_none() {
        let mut items = vec![Some(1), None, Some(2), Some(3), Some(4), None, None];
        items.pop_while(|&x| x.is_none());
        assert_eq!(items, vec![Some(1), None, Some(2), Some(3), Some(4)]);
    }
}
