/// Given an integer `n`, return the number of prime numbers that are
/// strictly less than `n`.
struct Solution;

impl Solution {

    // TODO: Cut down on unnecessary calculations
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        match n {
            0 => 0,
            1 => 0,
            _ => {
                let mut status = vec![true; n + 1];
                status[0] = false;
                status[1] = false;

                for i in 2..n {
                    let mut mult = 2;
                    let mut value = i * mult;
                    while value <= n {
                        status[value] = false;
                        mult += 1;
                        value = i * mult;
                    }
                }
                // Strictly less than n!!
                status[n] = false;
                status.into_iter().filter(|s| *s).count() as i32
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 10;
        let result = Solution::count_primes(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let n = 0;
        let result = Solution::count_primes(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let n = 1;
        let result = Solution::count_primes(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn strictly_less() {
        let n = 2;
        let result = Solution::count_primes(n);
        assert_eq!(result, 0);
    }

}
