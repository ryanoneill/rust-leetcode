use std::collections::VecDeque;

/// You have a `RecentCounter` class which counts the number of recent requests
/// within a certain time frame.
///
/// Implement the `RecentCounter` class:
/// * `RecentCounter()` Initializes the counter with zero recent requests.
/// * `int ping(int t)` Adds a new request at time t, where t represents some
///   time in milliseconds, and returns the number of requests that has
///   happened in the past `3000` milliseconds (including the new request).
///   Specifically, return the number of requests that have happened in the
///   inclusive range `[t - 3000, t]`.
///
/// It is guaranteed that every call to `ping` uses a strictly larger value of `t`
/// than the previous call.
struct RecentCounter {
    pings: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            pings: VecDeque::new(),
        }
    }

    fn is_front_old(&self, t: i32) -> bool {
        match self.pings.front() {
            None => false,
            Some(p) => t - p > 3000,
        }
    }

    fn expire_old(&mut self, t: i32) {
        while self.is_front_old(t) {
            self.pings.pop_front();
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        self.expire_old(t);
        self.pings.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn example_1() {
        let mut recent_counter = RecentCounter::new();
        let result1 = recent_counter.ping(1);
        assert_eq!(result1, 1);
        let result2 = recent_counter.ping(100);
        assert_eq!(result2, 2);
        let result3 = recent_counter.ping(3001);
        assert_eq!(result3, 3);
        let result4 = recent_counter.ping(3002);
        assert_eq!(result4, 3);
    }
}
