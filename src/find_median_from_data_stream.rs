use std::{collections::BinaryHeap, cmp::Reverse};

struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>
}

impl MedianFinder {

    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new()
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        let max = self.max_heap.pop().unwrap();
        self.min_heap.push(Reverse(max));
        if self.min_heap.len() > self.max_heap.len() {
            let min = self.min_heap.pop().unwrap().0;
            self.max_heap.push(min);
        }
    }

    fn peek_max(&self) -> i32 {
        *self.max_heap.peek().unwrap_or(&0)
    }

    fn peek_min(&self) -> i32 {
        let value = self.min_heap.peek();
        if value.is_some() {
            value.unwrap().0
        } else { 0 }
    }

    fn find_median(&self) -> f64 {
        let result: f64;
        if self.max_heap.len() > self.min_heap.len() {
            result = self.peek_max() as f64;
        } else {
            result = (self.peek_max() + self.peek_min()) as f64 / 2.0;
        }
        result
    }


}

#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn example_1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        let result1 = median_finder.find_median();
        assert_eq!(result1, 1.5);
        median_finder.add_num(3);
        let result2 = median_finder.find_median();
        assert_eq!(result2, 2.0);
    }

}
