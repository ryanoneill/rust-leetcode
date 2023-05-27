/// The Tribonacci sequence Tn is defined as follows:
///
/// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
///
/// Given `n`, return the value of Tn.
struct Solution;

impl Solution {

    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut values = vec![0; n+1];
        for i in 0..=n {
            if i == 0 {
                values[0] = 0;
            } else if i == 1 {
                values[1] = 1;
            } else if i == 2 {
                values[2] = 1;
            } else {
                values[i] = values[i-1] + values[i-2] + values[i-3];
            }
        }

        values[n]
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let result = Solution::tribonacci(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let n = 25;
        let result = Solution::tribonacci(n);
        assert_eq!(result, 1389537);
    }

}
