struct HeapArray {
    items: Vec<i32>
}

impl HeapArray {

    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn from_vec(values: Vec<i32>) -> Self {
        let mut result = Self::new();
        for value in values {
            result.push(value);
        }
        result
    }

    pub fn push(&mut self, value: i32) {
        self.items.push(value);
        let n = self.items.len();
        self.fix_bottom_up(n-1);
    }

    fn fix_bottom_up(&mut self, i: usize) {
        if i != 0 {
            let parent = (i - 1) / 2;

            if self.items[parent] < self.items[i] {
                self.items.swap(i, parent);
                if parent != 0 {
                    self.fix_bottom_up(parent);
                }
            }
        }
    }

    fn fix_top_down(&mut self, i: usize) {
        let n = self.items.len();

        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && self.items[left] > self.items[largest] {
            largest = left;
        }

        if right < n && self.items[right] > self.items[largest] {
            largest = right;
        }

        if largest != i {
            self.items.swap(i, largest);
            self.fix_top_down(largest);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let n = self.items.len();
        match n {
            0 => None,
            1 => self.items.pop(),
            _ => {
                self.items.swap(0, n-1);
                let result = self.items.pop();
                self.fix_top_down(0);
                result
            }
        }
    }

    // Example only for testing
    pub fn sort(&mut self) -> Vec<i32> {
        let mut result = Vec::new();
        let n = self.items.len();
        for _ in 0..n {
            let value = self.pop().unwrap();
            result.push(value);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::HeapArray;

    #[test]
    fn example_1() {
        let items = vec![5,4,3,1,7,8,9,2,6];
        let mut heap = HeapArray::from_vec(items);
        let result = heap.sort();
        assert_eq!(result, vec![9,8,7,6,5,4,3,2,1]);
    }

    #[test]
    fn example_2() {
        let items = vec![];
        let mut heap = HeapArray::from_vec(items);
        let result = heap.sort();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn example_3() {
        let items = vec![1,2,3,4];
        let mut heap = HeapArray::from_vec(items);
        let result = heap.sort();
        assert_eq!(result, vec![4,3,2,1]);
    }

}
