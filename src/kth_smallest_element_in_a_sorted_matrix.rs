use std::collections::BinaryHeap;

struct KthSmallestHeap {
    k: usize,
    heap: BinaryHeap<i32>,
}

impl KthSmallestHeap {
    fn new(k: usize) -> Self {
        Self {
            k,
            heap: BinaryHeap::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.heap.len() >= self.k
    }

    fn peek(&self) -> i32 {
        self.heap.peek().copied().unwrap_or_default()
    }

    fn push(&mut self, num: i32) -> bool {
        let mut result = false;
        if !self.is_full() {
            self.heap.push(num);
            result = true;
        } else if num < self.peek() {
            self.replace(num);
            result = true;
        }
        result
    }

    fn replace(&mut self, num: i32) {
        self.heap.pop();
        self.heap.push(num);
    }
}

/// Given an `n x n` `matrix` where each of the rows and columns is sorted in
/// ascending order, return the `kth` smallest element in the matrix.
///
/// Note that it is the `kth` smallest element in the sorted order, not the
/// `kth` distinct element.
///
/// You must find a solution with a memory complexity better than `O(n^2)`.
struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = matrix.len();
        let mut heap = KthSmallestHeap::new(k);
        let row_max = k.min(n);

        for i in 0..n {
            for j in 0..row_max {
                if !heap.push(matrix[i][j]) {
                    break;
                }
            }
        }

        heap.peek()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        let result = Solution::kth_smallest(matrix, k);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![-5]];
        let k = 1;
        let result = Solution::kth_smallest(matrix, k);
        assert_eq!(result, -5);
    }
}
