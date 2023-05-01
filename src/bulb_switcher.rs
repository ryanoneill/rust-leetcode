/// There are `n` bulbs that are initially off. You first turn on all the
/// bulbs, then you turn off every second bulb.
///
/// On the third round, you toggle every third bulb (turning on if it's off or
/// turning off if it's on). For the `ith` round, you toggle every `i` bulb.
/// For the `nth` round, you only toggle the last bulb.
///
/// Return the number of bulbs that are on after `n` rounds.
struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt().floor() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let n = 0;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let n = 1;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn real_world_1() {
        let n = 10;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn real_world_2() {
        let n = 24;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn real_world_3() {
        let n = 25;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 5);
    }

    #[test]
    fn real_world_4() {
        let n = 99999999;
        let result = Solution::bulb_switch(n);
        assert_eq!(result, 9999);
    }
}
