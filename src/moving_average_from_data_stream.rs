use std::collections::VecDeque;

/// Given a stream of integers and a window size, calculate the moving average
/// of all integers in the sliding window.
///
/// Implement the `MovingAverage` class:
/// * `MovingAverage(int size)` Initializes the object with the size of the
///   window `size`.
/// * `double next(int val)` Returns the moving average of the last `size`
///   values of the stream.
///
struct MovingAverage {
    items: VecDeque<i32>,
    window_size: usize,
    sum: i64
}

impl MovingAverage {

    fn new(size: i32) -> Self {
        MovingAverage {
            items: VecDeque::with_capacity(size as usize),
            window_size: size as usize,
            sum: 0
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.items.len() == self.window_size {
            self.items.pop_front().map(|i| {
                self.sum -= i as i64;
            });
        }
        self.items.push_back(val);
        self.sum += val as i64;

        self.sum as f64 / self.items.len() as f64
    }

}

#[cfg(test)]
mod tests {
    use super::MovingAverage;

    #[test]
    fn example_1() {
        let mut moving_average = MovingAverage::new(3);
        let result1 = moving_average.next(1);
        assert_eq!(result1, 1.0);
        let result2 = moving_average.next(10);
        assert_eq!(result2, 5.5);
        let result3 = moving_average.next(3);
        assert!(result3 - 4.66667 < f64::EPSILON);
        let result4 = moving_average.next(5);
        assert_eq!(result4, 6.0);
    }

}
