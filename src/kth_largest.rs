use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {

    pub fn new(k: usize) -> Self {
        Self {
            k,
            heap: BinaryHeap::new(),
        }
    }

    pub fn peek(&self) -> i32 {
        let value = self.heap.peek();
        if value.is_some() {
            value.unwrap().0
        } else {
            0
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.peek() {
            self.heap.push(Reverse(val));
            let _ = self.heap.pop();
        }
        self.peek()
    }

    fn add_items(&mut self, items: &Vec<i32>) -> i32 {
        let mut result = self.peek();
        for item in items {
            result = self.add(*item);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn example_1() {
        let items = vec![4, 5, 8, 2];
        let mut kth_largest = KthLargest::new(3);
        let result = kth_largest.add_items(&items);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let items = vec![1, 3, 5, 7, 9, 2, 4, 6];
        let mut kth_largest = KthLargest::new(4);
        let result = kth_largest.add_items(&items);
        assert_eq!(result, 5);
    }

}
