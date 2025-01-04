struct Solution;

impl Solution {

    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;

        let mut results = vec![0; high + 1];
        results[0] = 1;

        for i in 1..=high {
            let mut value: i64 = 0;
            if i >= zero {
                value += results[i - zero] as i64;
            }
            if i >= one {
                value += results[i - one] as i64;
            }
            value = value % 1000000007;
            results[i] = value as i32;
        }
        let mut result: i64 = 0;
        for j in low..=high {
            result += results[j] as i64;
            result = result % 1000000007;
        }
        result as i32
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        let result = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        let result = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(result, 5);
    }

}
