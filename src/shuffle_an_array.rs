use rand::prelude::*;

struct Solution {
    nums: Vec<i32>
}

impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.iter().copied().collect()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();

        let n = self.nums.len();

        let mut result = self.reset();

        for i in 0..n {
            let swap = rng.gen_range(i, n);
            result.swap(i, swap);
        }

        result
    }

}
