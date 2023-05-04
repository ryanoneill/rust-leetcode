use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct State {
    count: usize,
    row: usize,
}

impl State {

    fn new(count: usize, row: usize) -> Self {
        Self { count, row }
    }

}

/// You are given an `m x n` matrix `mat` of `1`'s (representing soldiers) and
/// `0`'s (representing civilians). The soldiers are positioned in front of the
/// civilians. That is, all the `1`'s will appear to the left of all the `0`'s
/// in each row.
///
/// A row `i` is weaker than a row `j` if one of the follwing is true:
///
/// * The number of soldiers in row `i` is less than the number of soldiers in
///   row `j`.
///
/// * Both rows have the same number of soldiers and `i < j`.
///
/// Return the indices of the `k` weakest rows in the matrix ordered from
/// weakest to strongest.
struct Solution;

impl Solution {

    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut min_heap = BinaryHeap::new();
        for (i, row) in mat.iter().enumerate() {
            let count = row.iter().take_while(|&&x| x == 1).count();
            let state = State::new(count, i);
            min_heap.push(Reverse(state));
        }

        let k = k as usize;
        let mut result = vec![0; k];
        for i in 0..k {
            let item = min_heap.pop().unwrap().0;
            result[i] = item.row as i32;
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mat = vec![vec![1,1,0,0,0], vec![1,1,1,1,0], vec![1,0,0,0,0], vec![1,1,0,0,0], vec![1,1,1,1,1]];
        let k = 3;
        let result = Solution::k_weakest_rows(mat, k);
        assert_eq!(result, vec![2,0,3]);
    }

    #[test]
    fn example_2() {
        let mat = vec![vec![1,0,0,0], vec![1,1,1,1], vec![1,0,0,0], vec![1,0,0,0]];
        let k = 2;
        let result = Solution::k_weakest_rows(mat, k);
        assert_eq!(result, vec![0, 2]);
    }

}
